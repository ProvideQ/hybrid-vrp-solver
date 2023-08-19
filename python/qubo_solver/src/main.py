import datetime
from dimod import BINARY, BinaryQuadraticModel
from dwave.system import DWaveSampler, EmbeddingComposite
from datetime import datetime


def main():
    # Your main code logic goes here
        
    with open('./att48_0.coo') as problem:
        bqm = BinaryQuadraticModel.from_coo(problem, vartype=BINARY)
        len(bqm)
        last = datetime.now().timestamp()
        print("started")
        sampleset = EmbeddingComposite(DWaveSampler()).sample(bqm, num_reads=1000, label='DWaveSampler with embedding num_reads=1000')
        # future.wait()
        now = datetime.now().timestamp()
        print(f"ended after {now - last}")
        # sampleset = future.result()['sampleset']
        print(sampleset.first.energy)
        print(sampleset.first.sample)

if __name__ == "__main__":
    main()