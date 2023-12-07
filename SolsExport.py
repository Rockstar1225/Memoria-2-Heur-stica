from itertools import product

class ArgsExporter:
    
    def __init__(self, sols: list, name_file: str, rows: int, cols: int, vehicles:dict):
        self.solutions = sols
        self.file = name_file
        self.ve = vehicles 
        self.matrix = [[["-" for i in range(cols)] for i in range(rows)] for i in range(len(self.solutions))] 
    
    def import_solutions(self):

        for i in range(len(self.solutions)):
            for key in self.solutions[i].keys():
                pos = self.solutions[i][key] 
                index = self.ve["id"].index(key)
                self.matrix[i][pos[0]][pos[1]] = key+"-"+self.ve["type"][index]+"-"+self.ve["c"][index]
    
    def export_sols(self):
        
        f = self.file[:-4]
        f += ".csv"
        
        self.import_solutions()

        try:
            with open(f,"w",encoding="utf-8") as file:
                file.write("\"N. Sol:\""+", "+str(len(self.matrix))+"\n")

                for i in range(len(self.matrix)):
                    for j in range(len(self.matrix[i])):
                        for k in range(len(self.matrix[i][j])):
                            if k != len(self.matrix[i][j]) -1:
                                file.write("\""+self.matrix[i][j][k]+"\",")
                            else:
                                file.write("\""+self.matrix[i][j][k]+"\"\n")
                    file.write("\n")
                    
        except Exception as e:
            print(f"Excepcion: {e}")
            print("Los datos no se pueden exportar a un archivo de salida!!")

        
