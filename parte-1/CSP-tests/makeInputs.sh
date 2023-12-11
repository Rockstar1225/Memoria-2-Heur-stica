#!/bin/bash

echo "+----------------------------+"
echo "|    Test  Inputs  Maker     |"
echo "+----------------------------+"
echo 
echo "Creando Archivo:  \"input1.txt\"..."

echo
echo "Caracteristicas :"
echo "   - Tamaño: 2x1"
echo "   - Numero de posiciones electricas: 2"
echo "   - Numero de vehiculos con congelador: 2"
echo "   - Posiciones elctricas == nº de vehiculos y posiciones consecutivas."
echo "   - Numero de vehiculos: 2."
echo 
echo "Caso de uso: mismo número de posiciones que de vehículos electricos."

touch input1.txt
echo "2x1" >> input1.txt
echo "PE: (1,1) (2,1)" >> input1.txt
echo "1-TSU-C" >> input1.txt
echo "2-TSU-C" >> input1.txt

sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input2.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 5x6"
echo "   - Numero de posiciones electricas: 4"
echo "   - Numero de vehiculos con congelador: 3"
echo "   - Posiciones Aleatorias con electricidad no consecutivas."
echo "   - Numero de vehiculos: 3."
echo 
echo "Caso de uso: menos vehiculos con congelador que plazas electricas."

touch input2.txt
echo "5x6" >> input2.txt
echo "PE: (1,1) (1,2) (5,1) (5,2)" >> input2.txt
echo "1-TSU-C" >> input2.txt
echo "2-TNU-C" >> input2.txt
echo "3-TSU-C" >> input2.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input3.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 3x3"
echo "   - Numero de posiciones electricas: 4"
echo "   - Numero de vehiculos con congelador: 4"
echo "   - Posiciones con electricidad no consecutivas."
echo "   - Numero de vehiculos: 4."
echo 
echo "Caso de uso: igual numero de vehiculos con congelador y plazas electricas."

touch input3.txt
echo "3x3" >> input3.txt
echo "PE: (1,1) (1,3) (3,1) (3,3)" >> input3.txt
echo "1-TSU-C" >> input3.txt
echo "2-TNU-C" >> input3.txt
echo "3-TNU-C" >> input3.txt
echo "4-TNU-C" >> input3.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input4.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 6x4"
echo "   - Numero de posiciones electricas: 6"
echo "   - Numero de vehiculos con congelador: 6"
echo "   - Posiciones Aleatorias con electricidad consecutivas."
echo "   - Numero de vehiculos: 6."
echo 
echo "Caso de uso: posiciones consecutivas de plazas electricas."

touch input4.txt
echo "6x4" >> input4.txt
echo "PE: (1,2) (2,2) (3,2) (4,4) (5,4) (6,4)" >> input4.txt
echo "1-TSU-C" >> input4.txt
echo "2-TNU-C" >> input4.txt
echo "3-TNU-C" >> input4.txt
echo "4-TNU-C" >> input4.txt
echo "5-TSU-C" >> input4.txt
echo "6-TNU-C" >> input4.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input5.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 3x3"
echo "   - Numero de posiciones electricas: 6"
echo "   - Numero de vehiculos con congelador: 6"
echo "   - Posiciones con electricidad consecutivas."
echo "   - Numero de vehiculos: 6."
echo 
echo "Caso de uso: Caso para posiciones consecutivas en los bordes."

touch input5.txt
echo "3x3" >> input5.txt
echo "PE: (1,1) (2,1) (2,2) (3,2) (1,3) (2,3)" >> input5.txt
echo "1-TSU-C" >> input5.txt
echo "2-TNU-C" >> input5.txt
echo "3-TNU-C" >> input5.txt
echo "4-TNU-C" >> input5.txt
echo "5-TSU-C" >> input5.txt
echo "6-TNU-C" >> input5.txt
sleep 2
echo "-----------------------------------------------------------------"

