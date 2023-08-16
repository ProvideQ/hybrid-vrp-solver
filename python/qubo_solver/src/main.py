import datetime
from dimod import BINARY, BinaryQuadraticModel
from dwave.cloud import Client
from datetime import datetime


def main():
    # Your main code logic goes here
    with Client.from_config() as client:
        solver = client.get_solver()
        
        with open('./att48_1.coo') as problem:
            bqm = BinaryQuadraticModel.from_coo(problem, vartype=BINARY)
            len(bqm)
            last = datetime.now().timestamp()
            print("started")
            future = solver.sample_bqm(bqm, time_limit=3)
            future.wait()
            now = datetime.now().timestamp()
            print(f"ended after {now - last}")
            sampleset = future.result()['sampleset']
            print(sampleset.first.energy)
            print(sampleset.first.sample)

if __name__ == "__main__":
    main()