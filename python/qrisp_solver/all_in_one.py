from math import ceil, factorial, log
from typing import Any, Tuple

import numpy as np
from qrisp import *
from qrisp.core import demux
from qrisp.environments import invert
from qrisp.grover import grovers_alg


# Create a function that generates a state of superposition of all permutations
def swap_to_front(qa, index):
    with invert():
        # The keyword ctrl_method = "gray_pt" allows the controlled swaps to be synthesized
        # using Margolus gates. These gates perform the same operation as a regular Toffoli
        # but add a different phase for each input. This phase will not matter though,
        # since it will be reverted once the ancilla values of the oracle are uncomputed.
        demux(qa[0], index, qa, permit_mismatching_size=True, ctrl_method="gray_pt")


def eval_perm(perm_specifiers, city_amount):
    N = len(perm_specifiers)

    # To filter out the cyclic permutations, we impose that the first city is always city 0
    # We will have to consider this assumption later when calculating the route distance
    # by manually adding the trip distance of the first trip (from city 0) and the
    # last trip (to city 0)
    qa = QuantumArray(QuantumFloat(int(np.ceil(np.log2(city_amount)))), city_amount - 1)

    qa[:] = np.arange(1, city_amount)

    for i in range(N):
        swap_to_front(qa[i:], perm_specifiers[i])

    return qa


# Create function that returns QuantumFloats specifying the permutations (these will be in uniform superposition)
def create_perm_specifiers(city_amount, init_seq=None):
    perm_specifiers = []

    for i in range(city_amount - 1):
        qf_size = int(np.ceil(np.log2(city_amount - i)))

        if i == 0:
            continue

        temp_qf = QuantumFloat(qf_size)

        if not init_seq is None:
            temp_qf[:] = init_seq[i - 1]

        perm_specifiers.append(temp_qf)

    return perm_specifiers


def qdict_calc_perm_travel_distance_forward(
    itinerary: QuantumArray,
    precision: int,
    city_amount: int,
    distance_matrix: np.ndarray[Any, np.dtype[np.float64]],
    demand: np.ndarray[Any, np.dtype[np.int64]],
    capacity: int,
) -> Tuple[QuantumFloat, QuantumArray, QuantumFloat]:
    # A QuantumFloat with n qubits and exponent -n
    # can represent values between 0 and 1
    res = QuantumFloat(precision * 2, -precision)

    # Fill QuantumDictionary with values
    qd = QuantumDictionary(return_type=res)
    for i in range(city_amount):
        for j in range(city_amount):
            qd[i, j] = res.truncate(distance_matrix[i, j])

    qd_to_zero = QuantumDictionary(return_type=res)

    for i in range(city_amount):
        qd_to_zero[i] = res.truncate(distance_matrix[0, i])

    res += qd_to_zero[itinerary[0]]

    # Add the distance of the final trip
    final_trip_distance = qd_to_zero[itinerary[-1]]
    res += final_trip_distance
    final_trip_distance.uncompute(recompute=True)

    bit_number_cap = ceil(log(capacity)) + 1

    demand_count_type = QuantumFloat(bit_number_cap)

    qa = QuantumDictionary(return_type=demand_count_type)
    for i in range(city_amount):
        qa[i] = demand[i]

    demand_indexer = QuantumFloat(1)
    demand_counter = QuantumArray(qtype=demand_count_type)
    demand_counter[:] = [0, 0]
    # print(demand_counter)

    demand_indexer[:] = 0
    with demand_counter[demand_indexer] as demand:
        demand += qa[itinerary[0]]

    # Evaluate result
    for i in range(city_amount - 2):
        demand_index = itinerary[(i + 1) % city_amount]
        # print(demand_index)
        city_demand = qa[demand_index]
        # print(city_demand)

        capped: QuantumBool

        with demand_counter[demand_indexer] as demand:
            demand += city_demand
            capped = demand <= capacity

        # print(capped)
        # print(demand_counter)

        with capped:
            trip_distance = qd[itinerary[i], demand_index]
            res += trip_distance
            trip_distance.uncompute(recompute=True)
        capped.flip()
        # print(capped)
        with capped:
            with demand_counter[demand_indexer] as demand:
                demand -= city_demand
            demand_indexer += 1
            with demand_counter[demand_indexer] as demand:
                demand += city_demand

            long_first_trip_distance = qd_to_zero[itinerary[i]]
            res += long_first_trip_distance
            long_first_trip_distance.uncompute(recompute=True)
            long_second_trip_distance = qd_to_zero[itinerary[(i + 1) % city_amount]]
            res += long_second_trip_distance
            long_second_trip_distance.uncompute(recompute=True)
        # print(demand_counter)
        with demand_counter[demand_indexer] as demand:
            with demand == city_demand:
                capped.flip()

        capped.delete()  # already verfied once
        city_demand.uncompute()

    return res, demand_counter, demand_indexer


def qdict_calc_perm_travel_distance_backward(
    itinerary: QuantumArray,
    precision: int,
    city_amount: int,
    distance_matrix: np.ndarray[Any, np.dtype[np.float64]],
    demand: np.ndarray[Any, np.dtype[np.int64]],
    capacity: int,
    forward_result: Tuple[QuantumFloat, QuantumArray, QuantumFloat],
) -> None:
    res, demand_counter, demand_indexer = forward_result

    # Fill QuantumDictionary with values
    qd = QuantumDictionary(return_type=res)
    for i in range(city_amount):
        for j in range(city_amount):
            qd[i, j] = res.truncate(distance_matrix[i, j])

    qd_to_zero = QuantumDictionary(return_type=res)

    for i in range(city_amount):
        qd_to_zero[i] = res.truncate(distance_matrix[0, i])

    bit_number_cap = ceil(log(capacity)) + 1

    demand_count_type = QuantumFloat(bit_number_cap)

    qa = QuantumDictionary(return_type=demand_count_type)
    for i in range(city_amount):
        qa[i] = demand[i]

    # uncompute demand_counter
    for i in reversed(range(city_amount - 2)):
        demand_index = itinerary[(i + 1) % city_amount]
        city_demand = qa[demand_index]

        was_capped: QuantumBool

        with demand_counter[demand_indexer] as demand:
            demand -= city_demand
            was_capped = demand == 0

        with was_capped:
            demand_indexer -= 1

            long_first_trip_distance = qd_to_zero[itinerary[i]]
            res -= long_first_trip_distance
            long_first_trip_distance.uncompute(recompute=True)
            long_second_trip_distance = qd_to_zero[itinerary[(i + 1) % city_amount]]
            res -= long_second_trip_distance
            long_second_trip_distance.uncompute(recompute=True)
        was_capped.flip()
        with was_capped:
            trip_distance = qd[itinerary[i], demand_index]
            res -= trip_distance
            trip_distance.uncompute(recompute=True)

        with demand_counter[demand_indexer] as demand:
            added = demand + city_demand

            reverse_capped = added <= capacity

            added.uncompute(recompute=True)

        with reverse_capped:
            was_capped.flip()

        reverse_capped.uncompute()

        was_capped.delete(verify=True)  # verified

        city_demand.uncompute()

    last_demand = qa[itinerary[0]]
    with demand_counter[demand_indexer] as demand:
        demand -= last_demand

    last_demand.uncompute()

    demand_indexer.delete()  # verified
    demand_counter.delete()  # verified

    # remove the distance of the first and final trip
    first_trip_distance = qd_to_zero[itinerary[0]]
    res -= first_trip_distance
    first_trip_distance.uncompute(recompute=True)

    final_trip_distance = qd_to_zero[itinerary[-1]]
    res -= final_trip_distance
    final_trip_distance.uncompute(recompute=True)

    res.delete()  # verified


def eval_distance_threshold(
    perm_specifiers,
    precision,
    threshold,
    city_amount,
    distance_matrix,
    city_demand,
    max_cap,
):
    itinerary = eval_perm(perm_specifiers, city_amount=city_amount)

    distance, demand_array, demand_indexer = qdict_calc_perm_travel_distance_forward(
        itinerary, precision, city_amount, distance_matrix, city_demand, max_cap
    )

    is_below_treshold = distance <= threshold

    z(is_below_treshold)

    with distance <= threshold:
        is_below_treshold.flip()

    qdict_calc_perm_travel_distance_backward(
        itinerary,
        precision,
        city_amount,
        distance_matrix,
        city_demand,
        max_cap,
        forward_result=(distance, demand_array, demand_indexer),
    )

    itinerary.uncompute()


max_cap = 2

city_amount = 4

city_coords = np.array([[0, 0], [1, 0.5], [0.5, -1], [-2, 0.5]])


distance_matrix = np.array(
    [
        [np.linalg.norm(city_coords[i] - city_coords[j]) for i in range(city_amount)]
        for j in range(city_amount)
    ]
)

print(distance_matrix)

city_demand = np.array([0, 1, 1, 1])

perm_specifiers = create_perm_specifiers(city_amount)
for qv in perm_specifiers:
    h(qv)


winner_state_amount = 2 ** sum([qv.size for qv in perm_specifiers]) / factorial(
    city_amount - 2
)


grovers_alg(
    perm_specifiers,  # Permutation specifiers
    eval_distance_threshold,  # Oracle function
    kwargs={
        "threshold": 11,
        "precision": 5,
        "city_amount": city_amount,
        "distance_matrix": distance_matrix,
        "city_demand": city_demand,
        "max_cap": max_cap,
    },  # Specify the keyword arguments for the Oracle
    winner_state_amount=winner_state_amount,
)  # Specify the estimated amount of winners

res = multi_measurement(perm_specifiers)

print(res)


# for testing purposes WHEN adding verify=True to all delete calls and running this code it works
# perm = eval_perm(perm_specifiers, city_amount=city_amount)
# print(perm)

# result = qdict_calc_perm_travel_distance_forward(
#     perm, 2, city_amount, distance_matrix, city_demand, max_cap
# )
# print(result[0])

# qdict_calc_perm_travel_distance_backward(
#     perm, 2, city_amount, distance_matrix, city_demand, max_cap, forward_result=result
# )
