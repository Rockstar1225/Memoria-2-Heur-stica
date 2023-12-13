from collections.abc import Iterable
import re

class ArgParser:
    def __init__(self, ruta: str):
        try:
            with open(ruta, "r", encoding="utf-8") as file:
                lines = tuple(x[:-1] for x in file.readlines())
        except FileNotFoundError:
            print("La ruta de archivo inicial no existente!!\n")
            return

        print("\t+---------------------------+")
        print("\t|     Solver Practica 2     |")
        print("\t+---------------------------+\n")

        print(f"\tAnalisis de archivo {ruta}:")

        if (not self.parse_range(lines[0])) or (not self.parse_electric(lines[1])):
            print("El archivo no se ha reconocido debido a un error de cabecera!!!")
            return

        if not self.parse_vehicles(lines[2:]):
            print("El contenido del archivo no se puede parsear!!!")

    def parse_range(self, line: str) -> bool:
        try:
            rows, cols = line.split("x")
            self.rows = int(rows)
            self.cols = int(cols)
            print(f"\t\t - Tamaño: {self.rows}x{self.cols}")
        except ValueError:
            return False
        return True

    def parse_electric(self, line: str) -> bool:
        try:
            self.electric = tuple((int(row), int(col)) for row, col in re.findall(r"\((\d+),(\d+)\)", line[3:]))
            print(f"\t\t - Lista de posiciones electricas: {self.electric}")
        except ValueError:
            return False
        return True

    def parse_vehicles(self, lines: Iterable[str]) -> bool:
        self.vehicles = {id: (vehicle_type, energy) for id, vehicle_type, energy in (vehicle.split("-") for vehicle in lines)}
        print(f"\t\t - Tabla de vehiculos: {self.vehicles}\n")
        return True

    def len_vehicles(self):
        return len(self.vehicles)

    def group_by_type(self, vehicle_type: str) -> Iterable[str]:
        return (id for id, properties in self.vehicles.items() if properties[0] == vehicle_type)

    def group_by_energy(self, energy: str) -> Iterable[str]:
        return (id for id, properties in self.vehicles.items() if properties[1] == energy)
