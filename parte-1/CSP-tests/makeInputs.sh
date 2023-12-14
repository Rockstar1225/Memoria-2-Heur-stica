#!/bin/bash

echo "+----------------------------+"
echo "|    Test  Inputs  Maker     |"
echo "+----------------------------+"
echo 

echo "Creando Archivo:  \"input1\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 1x2"
echo "   - Numero de vehiculos: 1"
echo 
echo "Caso de uso: sin solución (parking demasiado pequeño)"

touch input1
echo "1x2" >> input1
echo "PE:(1,1)" >> input1
echo "1-TSU-X" >> input1
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input2\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 2x1"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 2"
echo "   - Posiciones eléctricas == nº de vehiculos y posiciones consecutivas"
echo "   - Numero de vehiculos: 2"
echo 
echo "Caso de uso: sin solución (vehículos consecutivos contra un borde del parking)"

touch input2
echo "2x1" >> input2
echo "PE:(1,1)(2,1)" >> input2
echo "1-TSU-C" >> input2
echo "2-TSU-C" >> input2
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input3\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x1"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 2"
echo "   - Posiciones eléctricas == nº de vehiculos y posiciones consecutivas"
echo "   - Numero de vehiculos: 2"
echo 
echo "Caso de uso: sin solución (vehículos consecutivos contra un borde del parking)"

touch input3
echo "3x1" >> input3
echo "PE:(1,1)(2,1)" >> input3
echo "1-TSU-C" >> input3
echo "2-TSU-C" >> input3
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input4\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x1"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 2"
echo "   - Posiciones eléctricas == nº de vehiculos y posiciones consecutivas"
echo "   - Numero de vehiculos: 2"
echo 
echo "Caso de uso: sin solución (vehículos consecutivos contra un borde del parking)"

touch input4
echo "3x1" >> input4
echo "PE:(2,1)(3,1)" >> input4
echo "1-TSU-C" >> input4
echo "2-TSU-C" >> input4
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input5\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 4x1"
echo "   - Numero de posiciones eléctricas: 3"
echo "   - Numero de vehiculos con congelador: 3"
echo "   - Numero de vehiculos: 3"
echo "   - Posiciones eléctricas == nº de vehiculos y posiciones consecutivas en una columna"
echo 
echo "Caso de uso: sin solución (3 vehículos consecutivos en una columna)"

touch input5
echo "4x1" >> input5
echo "PE:(1,1)(2,1)(3,1)" >> input5
echo "1-TSU-C" >> input5
echo "2-TSU-C" >> input5
echo "3-TSU-C" >> input5
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input6\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x1"
echo "   - Numero de vehiculos: 4"
echo 
echo "Caso de uso: sin solución (no hay suficientes plazas distintas)"

touch input6
echo "3x1" >> input6
echo "PE:(1,1)" >> input6
echo "1-TSU-X" >> input6
echo "2-TSU-X" >> input6
echo "3-TSU-X" >> input6
echo "4-TSU-X" >> input6
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input7\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x1"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 3"
echo 
echo "Caso de uso: sin solución (no hay suficientes plazas eléctricas distintas)"

touch input7
echo "3x1" >> input7
echo "PE:(1,1)(2,2)" >> input7
echo "1-TSU-C" >> input7
echo "2-TSU-C" >> input7
echo "3-TSU-C" >> input7
sleep 2
echo "-----------------------------------------------------------------"

echo "Creando Archivo:  \"input8\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 2x2"
echo "   - Numero de posiciones eléctricas: 1"
echo "   - Numero de vehiculos con congelador: 1"
echo "   - Numero de vehiculos: 2"
echo "   - Única posición eléctrica en la primera columna"
echo 
echo "Caso de uso: Un vehículo TNU no puede estar en la misma fila a la derecha de uno TSU"

touch input8
echo "2x2" >> input8
echo "PE:(1,1)" >> input8
echo "1-TSU-C" >> input8
echo "2-TNU-X" >> input8
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input9\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 2x2"
echo "   - Numero de posiciones eléctricas: 1"
echo "   - Numero de vehiculos con congelador: 1"
echo "   - Numero de vehiculos: 2"
echo "   - Única posición eléctrica en la primera columna"
echo 
echo "Caso de uso: Un vehículo TSU sí puede estar en la misma fila a la derecha de otro TSU"

touch input9
echo "2x2" >> input9
echo "PE:(1,1)" >> input9
echo "1-TSU-C" >> input9
echo "2-TSU-X" >> input9
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input10\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 2x2"
echo "   - Numero de posiciones eléctricas: 1"
echo "   - Numero de vehiculos con congelador: 1"
echo "   - Numero de vehiculos: 2"
echo "   - Única posición eléctrica en la primera columna"
echo 
echo "Caso de uso: Un vehículo TNU sí puede estar en la misma fila a la izquierda de uno TSU"

touch input10
echo "2x2" >> input10
echo "PE:(1,2)" >> input10
echo "1-TSU-C" >> input10
echo "2-TNU-X" >> input10
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input11\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x3"
echo "   - Numero de posiciones eléctricas: 3"
echo "   - Numero de vehiculos con congelador: 2"
echo "   - Numero de vehiculos: 2"
echo 
echo "Caso de uso: menos vehiculos con congelador que plazas eléctricas"

touch input11
echo "3x3" >> input11
echo "PE:(1,1)(2,2)(3,3)" >> input11
echo "1-TNU-C" >> input11
echo "2-TNU-C" >> input11
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input12\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 2x2"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 0"
echo "   - Numero de vehiculos: 2"
echo 
echo "Caso de uso: los vehículos sin congelador pueden usar plazas eléctrucas libres"

touch input12
echo "2x2" >> input12
echo "PE:(1,1)(2,2)" >> input12
echo "1-TNU-X" >> input12
echo "2-TNU-X" >> input12
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input13\"..."
echo
echo "Caracteristicas:"
echo "   - Tamaño: 3x2"
echo "   - Numero de posiciones eléctricas: 2"
echo "   - Numero de vehiculos con congelador: 1"
echo "   - Posiciones con electricidad consecutivas"
echo "   - Numero de vehiculos: 3"
echo 
echo "Caso de uso: caso general con soluciones"

touch input13
echo "3x2" >> input13
echo "PE:(1,1)(2,1)" >> input13
echo "1-TSU-C" >> input13
echo "2-TNU-X" >> input13
echo "3-TNU-X" >> input13
sleep 2
echo "-----------------------------------------------------------------"
