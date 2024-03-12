from math import ceil, log
from typing import Any

import numpy as np
from qrisp import (
    QuantumArray,
    QuantumBool,
    QuantumDictionary,
    QuantumFloat,
    cx,
    quantum_condition,
)


def qdict_calc_perm_travel_distance(
    itinerary: QuantumArray,
    precision: int,
    city_amount: int,
    distance_matrix: np.ndarray[Any, np.dtype[np.float64]],
    demand: np.ndarray[Any, np.dtype[np.int64]],
    capacity: int,
) -> QuantumFloat:
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

    res = qd_to_zero[itinerary[0]]

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

        capped = QuantumBool()

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
        city_demand.uncompute()
        capped.uncompute()

    return res
