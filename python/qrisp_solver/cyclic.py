from qrisp import *

b = QuantumFloat(3)
b[:] = {1: 0.5, 4: 0.5}


@auto_uncompute
def circle(inner_b: QuantumFloat):
    c = QuantumFloat(3)

    a = QuantumFloat(3)

    a += inner_b
    inner_b += a
    a += inner_b

    c += a

    a -= inner_b
    inner_b -= a
    a -= inner_b

    a.delete(verify=True)

    return c


outer_c = QuantumFloat(3)
outer_a = QuantumFloat(3)

func = redirect_qfunction(circle)

func(b, target=outer_c)


print(outer_c)

# c = QuantumFloat(3)
# h(c)

# a = QuantumFloat(3)

# test = b < c

# with test:
#     a += c
    
    
# with b < c:
#     test.flip()
    
    
# print(test)

# test.delete(verify=True)
    


# print(a)

# b.uncompute()
# a.uncompute()


# print(d.qs.compile())
