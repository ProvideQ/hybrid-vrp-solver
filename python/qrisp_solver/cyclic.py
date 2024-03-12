from qrisp import *

b = QuantumFloat(3)
b[:] = 1


@auto_uncompute
def circle(inner_b: QuantumFloat):
    c = QuantumFloat(3)
    c[:] = 1

    a = QuantumFloat(3)

    a += inner_b
    inner_b += a
    a += inner_b

    c += a

    a.delete()

    return c


c = circle(b)

# b.uncompute()
# a.uncompute()


print(c.qs)
