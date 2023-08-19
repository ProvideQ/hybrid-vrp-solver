import datetime
from dimod import BINARY, BinaryQuadraticModel
from dwave.system import DWaveSampler, EmbeddingComposite
from datetime import datetime
from hybrid import SimplifiedQbsolv, State


def main():
    # Your main code logic goes here
        
    with open('./att48_0.coo') as problem:
        bqm = BinaryQuadraticModel.from_coo(problem, vartype=BINARY)
        len(bqm)
        last = datetime.now().timestamp()
        print("started")
        workflow = SimplifiedQbsolv(max_time=10)
        # future.wait()
        init_state = State.from_problem(bqm)
        final_state = workflow.run(init_state, label="QBsolv").result()
        now = datetime.now().timestamp()
        print(f"ended after {now - last}")
        sampleset = final_state.samples
        print(sampleset.first.energy)
        print(sampleset.first.sample)

if __name__ == "__main__":
    main()