#!/bin/bash

inputs=("input1" "input2" "input3" "input4" "input5" "input6" "input7" "input8" "input9" "input10" "input11" "input12" "input13")

cd ./CSP-tests
echo "Otorgar permiso de ejecucion a script creador de tests."
chmod +x makeInputs.sh
for i in "${inputs[@]}"; do
    rm $i
done



./makeInputs.sh
cd ..

echo
echo "+---------------------+"
echo "|  Tests Funcionales  |"
echo "+---------------------+"
echo


for ((i = 0; i < ${#inputs[@]}; i++)); do
  
  archivo=$(($i + 1))
  # echo

  python3 ./CSPParking.py ./CSP-tests/${inputs[i]} >> ./CSP-tests/out.a 
  # echo "Comprobando salida de archivo ${inputs[i]}."
  # len=$(wc -l < ./CSP-tests/${inputs[i]}.csv)
  # echo "Longitud de archivo: $len"
  
  # if [ $i -eq 0 ] || [ $i -eq 3 ] || [ $i -eq 4 ]; then
  #   
  #   if [ $((len)) -eq 1 ]; then
  #     echo "Resultado: Test archivo ${archivo} Exitoso!!!"
  #     echo
  #   else
  #     echo "Resultado: Test archivo ${archivo} fallido"
  #   fi
  # elif [ -s "./CSP-tests/${inputs[i]}.csv" ]; then
  #   echo "Resultado: Test archivo ${archivo} Exitoso!!!"
  #   echo
  # else
  #   echo "Resultado: Test archivo ${archivo} fallido :-("
  #   echo
  # fi 

done

rm CSP-tests/out.a
