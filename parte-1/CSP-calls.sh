#!/bin/bash

inputs=("input1.txt" "input2.txt" "input3.txt" "input4.txt" "input5.txt")
outputs=("input1.csv" "input2.csv" "input3.csv" "input4.csv" "input5.csv")

cd ./CSP-tests
echo "Otorgar permiso de ejecucion a script creador de tests."
sudo chmod +x makeInputs.sh
for i in "${inputs[@]}"; do
  rm $i
done

rm out.a


./makeInputs.sh
cd ..
sleep 10

echo
echo "+---------------------+"
echo "|  Tests Funcionales  |"
echo "+---------------------+"
echo


for ((i = 0; i < ${#inputs[@]}; i++)); do
  
  archivo=$(($i + 1))
  echo

  python ./CSPParking.py ./CSP-tests/${inputs[i]} >> ./CSP-tests/out.a 
  echo "Comprobando salida de archivo ${inputs[i]}."
  len=$(wc -l < ./CSP-tests/${outputs[i]})
  echo "Longitud de archivo: $len"
  
  if [ $i -eq 0 ] || [ $i -eq 3 ] || [ $i -eq 4 ]; then
    
    if [ $((len)) -eq 1 ]; then
      echo "Resultado: Test archivo ${archivo} Exitoso!!!"
      echo
    else
      echo "Resultado: Test archivo ${archivo} fallido"
    fi
  elif [ -s "./CSP-tests/${outputs[i]}" ]; then
    echo "Resultado: Test archivo ${archivo} Exitoso!!!"
    echo
  else
    echo "Resultado: Test archivo ${archivo} fallido :-("
    echo
  fi 

done
