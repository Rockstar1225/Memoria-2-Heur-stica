import sys

from itertools import combinations, product
from collections.abc import Collection
import constraint
from ArgsParser import ArgParser
from SolsExport import ArgsExporter


def consecutive(vals: Collection[int]): 
    return min(vals) + (len(vals) - 1) == max(vals)


def notPosSuperior(a: tuple, b: tuple):
    return a[0] != b[0] or a[1] > b[1]    

def notUpAndDown(b: tuple, a: tuple, c: tuple): 
    return not ( (b[1] == a[1] == c[1]) and consecutive((a[0], b[0], c[0])))

def notBounds(max_row: int, a: tuple, b: tuple):
    return a[1] != b[1] or not consecutive((a[0], b[0])) or min(a[0],b[0]) not in (1, max_row)

def main():
    # Comprobacion de argumentos
    if len(sys.argv) != 2:
        print("Los argumentos pasados no son los correctos!!")
        print("Ejemplo: python CSPParking.py <path>")
        return

    # Inicializando el parser de los argumentos
    ruta = sys.argv[1]
    parser = ArgParser(ruta)

    # lista de posiciones con electricidad 
    pos_elec = parser.electric
    # lista de todas las plazas
    pos = tuple((x,y) for x, y in product(range(1, parser.rows + 1), range(1, parser.cols + 1)))
    # tabla de vehículos disponibles
    vehicles = parser.vehicles

    print("\tParametros de simulacion:")
    print(f"\t\t - Dominio para los vehiculos con congelador: {pos_elec}")
    print(f"\t\t - Dominio para los vehiculos sin congelador: {pos}\n") 

    ########################################################################

    problem = constraint.Problem()

    # añadir como variables los coches con congelador.
    problem.addVariables(parser.group_by_energy("C"), pos_elec)

    # añadir como variables a los coches sin congelador.
    problem.addVariables(parser.group_by_energy("X"), pos)
       
    # añadir restricción: dos vehículos no pueden tener la misma plaza.
    problem.addConstraint(constraint.AllDifferentConstraint())

    # añadir restricción: si el parking solo tiene 1 fila, ningún vehículo
    # tendrá una casilla vacía arriba/abajo y el problema no tiene solución
    problem.addConstraint(lambda *_: parser.rows != 1)

    # añadir restricción: las posiciones de un tipo TSU son mayores a las de un dispositivo TNU.
    for x, y in product(parser.group_by_type("TSU"), parser.group_by_type("TNU")):
        problem.addConstraint(notPosSuperior, (x, y))

    # añadir restricción: tiene que haber una posición libre abajo o arriba de los extremos.
    for x, y in combinations(vehicles, 2):
        problem.addConstraint(lambda a, b: notBounds(parser.rows - 1, a, b), (x, y))

    # tiene que haber una posicion libre a los laterales de una intermedia.  
    for x, y, z in combinations(vehicles, 3):
        problem.addConstraint(notUpAndDown, (x, y, z)) 

    soluciones = problem.getSolutions()
    print("Longitud de soluciones:", len(soluciones)) 
    
    # Exportacion de las soluciones
    ArgsExporter(soluciones, ruta, parser.rows, parser.cols, vehicles).export_sols()

    ########################################################################

if __name__ == "__main__":
    main()
