#|/bin/bash

inputs=("input1.txt" "input2.txt" "input3.txt" "input4.txt")
outputs=("input1.csv" "input2.csv" "input3.csv" "input4.csv")

echo "Otorgar permiso de ejecucion a script creador de tests."
sudo chmod +x makeInputs.sh
for i in "${inputs[@]}"; do
  rm $i
done

rm out.a


./makeInputs.sh
echo
echo "+---------------------+"
echo "|  Tests Funcionales  |"
echo "+---------------------+"
echo


for ((i = 0; i < ${#inputs[@]}; i++)); do
  
  archivo=$(($i + 1))
  echo
  echo "$i ${inputs[i]}"

  python ./CSPParking.py ${inputs[i]} >> out.a 
  echo "Comprobando salida de archivo ${inputs[i]}."
  len=$(wc -l < ${outputs[i]})
  echo "Longitud de archivo: $len"
  
  if [ $i -eq 0 ]; then
    
    if [ $((len)) -eq 1 ]; then
      echo "Resultado: Test archivo ${archivo} Exitoso!!!"
      echo
    else
      echo "Resultado: Test archivo ${archivo} fallido"
    fi
  elif [ -s "${outputs[i]}" ]; then
    echo "Resultado: Test archivo ${archivo} Exitoso!!!"
    echo
  else
    echo "Resultado: Test archivo ${archivo} fallido :-("
    
  fi 

done

# for i in "${outputs[@]}"; do
  # rm $i
# done

