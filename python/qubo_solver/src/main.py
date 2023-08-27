import argparse
import os
from datetime import datetime
from typing import Literal

from dimod import BINARY, BinaryQuadraticModel
from dimod.serialization import coo
from dwave.cloud import Client
from solver import solve_with


def main():
    parser = argparse.ArgumentParser(
        prog="DWave QUBO solver",
        description="A CLI Program to initiate solving COOrdinate files with DWave Systems",
        epilog="Made by Lucas Berger for scientific purposes",
    )

    parser.add_argument("coo_file")
    parser.add_argument(
        "type", default="sim", choices=["sim", "hybrid", "qbsolv", "direct"]
    )
    parser.add_argument("--output-file")

    args = parser.parse_args()
    type: Literal["sim", "hybrid", "qbsolv", "direct"] = args.type

    with open(args.coo_file) as problem:
        bqm = coo.load(problem, vartype=BINARY)

        filename = os.path.basename(args.coo_file)

        last = datetime.now().timestamp()
        print("started")

        with Client.from_config() as _:
            now = datetime.now().timestamp()
            print(f"connected after {now - last}. starting solver")
            sampleset = solve_with(bqm, type, filename)

            # accessing the sampleset's properties await for the future
            print(sampleset.info)

            now = datetime.now().timestamp()
            print(f"ended {now - last}")

            if args.output_file:
                with open(args.output_file, "w") as out:
                    out.writelines(
                        [f"{bin}\n" for bin in sampleset.first.sample.values()]
                    )
            else:
                print(sampleset.first.energy)
                print(sampleset.first.sample)

        now = datetime.now().timestamp()
        print(f"connection closed after {now - last}")


if __name__ == "__main__":
    main()
