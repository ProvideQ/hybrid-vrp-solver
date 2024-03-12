from math import factorial

import numpy as np
from qrisp import *
from qrisp.grover import grovers_alg
from qrisp_solver.distance import qdict_calc_perm_travel_distance
from qrisp_solver.oracle import eval_distance_threshold
from qrisp_solver.permutation import create_perm_specifiers, eval_perm

max_cap = 2

city_amount = 4

city_coords = np.array([[0, 0], [1, 0.5], [0.5, -1], [-2, 0.5]])


distance_matrix = np.array(
    [
        [np.linalg.norm(city_coords[i] - city_coords[j]) for i in range(city_amount)]
        for j in range(city_amount)
    ]
)

city_demand = np.array([0, 1, 1, 1])

perm_specifiers = create_perm_specifiers(city_amount)
for qv in perm_specifiers:
    h(qv)
# perm = eval_perm(perm_specifiers, city_amount=city_amount)
# print(perm)


# test_itinerary = QuantumArray(qtype=QuantumFloat(3))
# test_itinerary[:] = [1, 2, 3]
# qdict_res = qdict_calc_perm_travel_distance(
#     test_itinerary, 5, city_amount, distance_matrix, city_demand, max_cap
# )
# print(qdict_res)

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

# print(res)
# qc = qdict_res.qs.compile()
# print(qdict_res.qs)
# print(qc.num_qubits())
