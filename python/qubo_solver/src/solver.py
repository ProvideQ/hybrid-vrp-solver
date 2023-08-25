from typing import Literal

from dimod import BinaryQuadraticModel, Sampler, SampleSet, SimulatedAnnealingSampler
from dwave.system import DWaveSampler, EmbeddingComposite, LeapHybridSampler
from hybrid import SimplifiedQbsolv, State

solvertype = Literal["sim", "hybrid", "qbsolv", "direct"]


def solve_with(bqm: BinaryQuadraticModel, type: solvertype, label: str) -> SampleSet:
    if type == "sim":
        sampler: Sampler = SimulatedAnnealingSampler()
        return sampler.sample(bqm)
    elif type == "direct":
        sampler: Sampler = EmbeddingComposite(DWaveSampler())
        return sampler.sample(
            bqm,
            num_reads=250,
            label=f"DWaveSampler with embedding num_reads=250 {label}",
        )
    elif type == "hybrid":
        sampler: Sampler = LeapHybridSampler()
        return sampler.sample(
            bqm, time_limit=10, label=f"LeapHybridSampler num_reads=250: {label}"
        )
    elif type == "qbsolv":
        init_state = State.from_problem(bqm)
        workflow = SimplifiedQbsolv()
        final_state = workflow.run(init_state).result()
        return final_state.samples
