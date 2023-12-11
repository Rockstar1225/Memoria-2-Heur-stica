class ArgParser:
    def __init__(self):
        self.__ruta = ""

        self.rows = 0
        self.cols = 0
        self.electric = []
        self.vehicles = {}

    def load_file(self):
        with open(self.__ruta, "r", encoding="utf-8") as file:
            lines = file.readlines()

            for i in range(len(lines)):
                lines[i] = lines[i][:-1]
            
            print("\t+---------------------------+")
            print("\t|     Solver Practica 2     |")
            print("\t+---------------------------+\n")

            print(f"\tAnalisis de archivo {self.__ruta}: \n")
                        



            if (not self.parse_range(lines[0])) or (not self.parse_electric(lines[1])):
                print("El archivo no se ha reconocido debido a un error de cabecera!!!\n")
                return

            lines.pop(0)
            lines.pop(0)

            if not self.parse_vehicles(lines):
                print("El contenido del archivo no se puede parsear!!!\n")

    def parse_range(self, line: str) -> bool:
        try:
            rows, cols= line.split("x")
            self.rows = int(rows)
            self.cols = int(cols)
            print(f"\t\t - Tamaño: {self.rows}x{self.cols}\n")
            
        except Exception:
            return False

        return True

    def parse_electric(self, line: str) -> bool:
        elects = line.split(" ")
        elects.pop(0)
        print(f"\t\t - Lista de posiciones electricas: {elects}\n")
        try:
            for cell in elects:
                vals = cell[1:-1]
                vals = vals.split(",")

                self.electric.append( ( int(vals[0])-1, int(vals[1])-1 ) )
        except Exception:
            return False
        return True

    def parse_vehicles(self, lines) -> bool:
        self.vehicles["id"] = []
        self.vehicles["type"] = []
        self.vehicles["c"] = []
    
        try:
            for l in lines:
                l_r = l.split("-")
                if not (l_r[0] in self.vehicles["id"]):
                    self.vehicles["id"].append(l_r[0])
                    self.vehicles["type"].append(l_r[1])
                    self.vehicles["c"].append(l_r[2])
        except Exception:
            return False
        
        print("\t\t - Tabla de vehiculos: ")
        print("\t\t\t + IDs: ",self.vehicles["id"])
        print("\t\t\t + Tipos: ",self.vehicles["type"])
        print("\t\t\t + Congelador: ",self.vehicles["c"],"\n")

        return True

    def len_vehicles(self):
        return len(self.vehicles["id"])

    def group_by_type(self, t:str):
        try:
            res = []
            lista = self.vehicles["type"]
            for i in range(len(lista)):
                if lista[i] == t:
                    res.append((self.vehicles["id"][i], self.vehicles["c"][i]))
            return res

        except Exception:

            print("No se pudo agrupar los datos por ese valor de tipo!!!\n")
            return []

    def group_by_c(self, c: str):
        try:
            res = []
            lista = self.vehicles["c"]
            for i in range(len(lista)):
                if lista[i] == c:
                    res.append((self.vehicles["id"][i], self.vehicles["type"][i]))
            return res

        except Exception:

            print("No se pudo agrupar los datos por ese valor de congelador!!!\n")
            return []

    @property
    def ruta(self):
        return self.__ruta

    @ruta.setter
    def ruta(self, value: str):
        try:
            with open(value, "r", encoding="utf-8") as file:
                if file.readable():
                    self.__ruta = value
                else:
                    print("El archivo indicado no se puede leer!!\n")
        except FileNotFoundError:
            print("La ruta de archivo inicial no existente!!\n")
