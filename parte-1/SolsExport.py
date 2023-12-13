MAX_NUM_SOLUTIONS = 20

class ArgsExporter:
    
    def __init__(self, sols: list, name_file: str, rows: int, cols: int, vehicles: dict[str, tuple[str, str]]):
        self.solutions = sols
        self.file = name_file + ".csv"
        self.vehicles = vehicles 
        self.matrix = [[["-" for _ in range(cols)] for _ in range(rows)] for _ in range(min(MAX_NUM_SOLUTIONS, len(self.solutions)))] 
        self.import_solutions()
    
    def import_solutions(self):
        for i in range(len(self.matrix)):
            for id, pos in self.solutions[i].items():
                self.matrix[i][pos[0] - 1][pos[1] - 1] = f"{id}-{self.vehicles[id][0]}-{self.vehicles[id][1]}"
    
    def export_sols(self):
        try:
            with open(self.file, "w", encoding="utf-8") as file:
                file.write(f"\"N. Sol:\",{len(self.solutions)}\n")
                for solution in self.matrix:
                    for row in solution:
                        file.write(",".join(f"\"{vehicle}\"" for vehicle in row) + "\n")
                    file.write("\n")

        except Exception as e:
            print(f"Excepcion: {e}")
            print("Los datos no se pueden exportar a un archivo de salida!!")
