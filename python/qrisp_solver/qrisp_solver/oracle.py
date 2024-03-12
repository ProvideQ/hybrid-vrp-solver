from qrisp import *

from .distance import qdict_calc_perm_travel_distance
from .permutation import eval_perm


@auto_uncompute
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

    distance = qdict_calc_perm_travel_distance(
        itinerary, precision, city_amount, distance_matrix, city_demand, max_cap
    )

    is_below_treshold = distance <= threshold

    z(is_below_treshold)
