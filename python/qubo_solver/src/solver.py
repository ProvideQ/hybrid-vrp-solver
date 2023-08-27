from datetime import datetime
from typing import Literal

from dimod import BinaryQuadraticModel, Sampler, SampleSet, SimulatedAnnealingSampler
from dwave.system import DWaveSampler, EmbeddingComposite, LeapHybridSampler
from hybrid import SimplifiedQbsolv, State

solvertype = Literal["sim", "hybrid", "qbsolv", "direct"]


def solve_with(bqm: BinaryQuadraticModel, type: solvertype, label: str) -> SampleSet:
    if type == "sim":
        last = datetime.now().timestamp()
        sampler: Sampler = SimulatedAnnealingSampler()
        print(f"sampler created took {datetime.now().timestamp() - last}")
        return sampler.sample(bqm)
    elif type == "direct":
        last = datetime.now().timestamp()
        sampler: Sampler = EmbeddingComposite(DWaveSampler())
        print(f"sampler created took {datetime.now().timestamp() - last}")
        return sampler.sample(
            bqm,
            num_reads=250,
            label=f"DWaveSampler with embedding num_reads=250 {label}",
        )
    elif type == "hybrid":
        last = datetime.now().timestamp()
        sampler: Sampler = LeapHybridSampler()
        print(f"sampler created took {datetime.now().timestamp() - last}")
        return sampler.sample(
            bqm, time_limit=5, label=f"LeapHybridSampler num_reads=250: {label}"
        )
    elif type == "qbsolv":
        last = datetime.now().timestamp()
        init_state = State.from_problem(bqm)
        workflow = SimplifiedQbsolv(max_iter=3, max_time=10)
        print(f"workflow created took {datetime.now().timestamp() - last}")
        final_state = workflow.run(init_state).result()
        print(workflow.timers)
        return final_state.samples
