#!/bin/bash

echo "+----------------------------+"
echo "|    Test  Inputs  Maker     |"
echo "+----------------------------+"
echo 
echo "Creando Archivo:  \"input1.txt\"..."

echo
echo "Caracteristicas :"
echo "   - Tamaño: 5x5"
echo "   - Numero de posiciones electricas: 3"
echo "   - Numero de vehiculos con congelador: 4"
echo "   - Posiciones Aleatorias con electricidad consecutivas."
echo "   - Numero de vehiculos: 8."
echo 
echo "Caso de uso: mas vehiculos con congelador que plazas electricas."

touch input1.txt
echo "5x5" >> input1.txt
echo "PE: (1,2) (1,4) (1,5)" >> input1.txt
echo "1-TSU-C" >> input1.txt
echo "2-TNU-X" >> input1.txt
echo "3-TNU-X" >> input1.txt
echo "4-TNU-C" >> input1.txt
echo "5-TSU-X" >> input1.txt
echo "6-TNU-X" >> input1.txt
echo "7-TNU-C" >> input1.txt
echo "8-TSU-C" >> input1.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input2.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 5x6"
echo "   - Numero de posiciones electricas: 6"
echo "   - Numero de vehiculos con congelador: 4"
echo "   - Posiciones Aleatorias con electricidad no consecutivas."
echo "   - Numero de vehiculos: 8."
echo 
echo "Caso de uso: menos vehiculos con congelador que plazas electricas."

touch input2.txt
echo "5x6" >> input2.txt
echo "PE: (1,1) (1,2) (2,1) (4,1) (5,1) (5,2)" >> input2.txt
echo "1-TSU-C" >> input2.txt
echo "2-TNU-X" >> input2.txt
echo "3-TSU-X" >> input2.txt
echo "4-TNU-C" >> input2.txt
echo "5-TSU-X" >> input2.txt
echo "6-TNU-X" >> input2.txt
echo "7-TSU-C" >> input2.txt
echo "8-TSU-C" >> input2.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input3.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 8x8"
echo "   - Numero de posiciones electricas: 6"
echo "   - Numero de vehiculos con congelador: 6"
echo "   - Posiciones Aleatorias con electricidad no consecutivas."
echo "   - Numero de vehiculos: 8."
echo 
echo "Caso de uso: igual numero de vehiculos con congelador y plazas electricas."

touch input3.txt
echo "8x8" >> input3.txt
echo "PE: (1,1) (2,2) (3,3) (4,4) (7,7) (8,8)" >> input3.txt
echo "1-TSU-C" >> input3.txt
echo "2-TNU-C" >> input3.txt
echo "3-TNU-X" >> input3.txt
echo "4-TNU-C" >> input3.txt
echo "5-TSU-C" >> input3.txt
echo "6-TNU-X" >> input3.txt
echo "7-TNU-C" >> input3.txt
echo "8-TSU-C" >> input3.txt
sleep 2
echo "-----------------------------------------------------------------"

echo
echo "Creando Archivo:  \"input4.txt\"..."
echo
echo "Caracteristicas :"
echo "   - Tamaño: 8x5"
echo "   - Numero de posiciones electricas: 6"
echo "   - Numero de vehiculos con congelador: 6"
echo "   - Posiciones Aleatorias con electricidad consecutivas."
echo "   - Numero de vehiculos: 8."
echo 
echo "Caso de uso: posiciones consecutivas de plazas electricas."

touch input4.txt
echo "8x8" >> input4.txt
echo "PE: (1,2) (2,2) (3,2) (4,4) (5,4) (6,4)" >> input4.txt
echo "1-TSU-C" >> input4.txt
echo "2-TNU-X" >> input4.txt
echo "3-TNU-C" >> input4.txt
echo "4-TNU-C" >> input4.txt
echo "5-TSU-C" >> input4.txt
echo "6-TNU-X" >> input4.txt
echo "7-TNU-C" >> input4.txt
echo "8-TSU-C" >> input4.txt
sleep 2
echo "-----------------------------------------------------------------"


