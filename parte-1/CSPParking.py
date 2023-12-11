import sys
import math

from itertools import combinations, product, pairwise
from typing import Iterable
from constraint import *
from ArgsParser import *
from SolsExport import *


parser = None

def consecutive(args: Iterable[int]): 
    return all(map(lambda x: x[1] == x[0] +1,pairwise(args)))


def notPosSuperior(a: tuple, b: tuple):
    return a[0] != b[0] or a[1] > b[1]    

def notUpAndDown(b:tuple, a:tuple, c:tuple ): 
    return not ( (b[1] == a[1] == c[1]) and consecutive(sorted((b[0],a[0],c[0]))) ) 

def notBounds(a:tuple, b:tuple):
    return a[1] != b[1] or not( consecutive( sorted(( a[0], b[0] )) ) 
                               and min(a[0],b[0]) in (0, parser.rows-2))

def main():
    # Comprobacion de argumentos
    if len(sys.argv) != 2:
        print("Los argumentos pasados no son los correctos!!\n")
        print("Ejemplo: python CSPParking.py <path>\n")
        return

    # Inicializando el parser de los argumentos
    global parser
    parser = ArgParser()
    parser.ruta = sys.argv[1]
    parser.load_file()

    # lista de coches con congelador 
    ve_cong= [str(elec[0]) for elec in parser.group_by_c("C")]
    
    print("\tParametros de simulacion:\n")

    print("\t\t - Variables -> Lista de vehiculos con congelador: ",ve_cong,"\n") 


    # lista de posiciones con electricidad 
    pos_elec = [elem for elem in parser.electric]
    print("\t\t - Dominio para los vehiculos con congelador: ",pos_elec,"\n")
    
    # lista de coches sin congelador 
    ve_sin = [str(elec[0]) for elec in parser.group_by_c("X")]

    print("\t\t - Variables -> Lista de vehiculos sin congelador: ",ve_sin,"\n")
    
    # lista de todas las plazas
    pos = [ (x,y) for x,y in product( range(parser.rows), range(parser.cols) )]
    
    print("\t\t - Dominio para los vehiculos sin congelador: ",pos,"\n") 
    

    # tabla de vehículos disponibles
    global vehicles 
    vehicles = parser.vehicles

    ########################################################################

    problem = Problem()

    # añadir como variables los coches con congelador.
     
    problem.addVariables(ve_cong, pos_elec)

    # añadir como variables a los coches sin congelador.
     
    problem.addVariables(ve_sin, pos)

    # añadir restriccion: los valores de ve_cong seran las plazas con electricidad
       
    # añadir restricción: dos vehículos no pueden tener la misma plaza.
    problem.addConstraint(AllDifferentConstraint())

    # se da por hecho que un coche no puede asignarse a dos plazas.

    # añadir restricción: los coches con congelador solo en plazas electricas (implementado en el dominio de las variables).
    
    # añadir restriccion: las posiciones de un tipo TSU son mayores a las de un dispositivo TNU.
    for x,y in product(
            ( elem for elem in range(len(vehicles["id"])) if vehicles["type"][elem]=="TSU"),
            ( elem for elem in range(len(vehicles["id"])) if vehicles["type"][elem]=="TNU") 
            ):
        problem.addConstraint(notPosSuperior,(vehicles["id"][x],vehicles["id"][y]))

    # añadir restricción: tiene que haver una posición libre abajo o arriba de los extremos.
    
    for x,y in combinations(vehicles["id"],2):
        problem.addConstraint(
           notBounds,(x,y) 
        )

    # tiene que haber una posicion libre a los laterales de una intermedia.  
    for x,y,z in combinations(vehicles["id"],3):
        problem.addConstraint(notUpAndDown,(x,y,z)) 

    soluciones = problem.getSolutions()
    print("Longitud de soluciones: ",len(soluciones)) 
    
    # Exportacion de las soluciones
    exporter = ArgsExporter(soluciones,parser.ruta,parser.rows,parser.cols,vehicles) 

    exporter.export_sols()

    ########################################################################

if __name__ == "__main__":
    main()
