---
title: |
  ![](Logo_UC3M.png){width=27.0em}  
  Memoria Práctica 1 Heurística y Optimización
author: |
  | Álvaro Guerrero Espinosa - 100472294
  | Adrian Cortázar - 100475860

lang: es
documentclass: report
geometry: "left=3cm,right=3cm,top=3cm,bottom=3cm"
toc: true
toc-depth: 3

header-includes: |
    ```{=latex}
    %\parindent=1.5em
    \parskip=1em
    \newcommand{\N}{\mathbb{N}}
    \newcommand{\R}{\mathbb{R}^+}
    ```
---

\clearpage

## 1 Introducción

El objetivo de la realización de esta práctica consiste en:

- Modelar un problema y resolverlo utilizando SCP
- Diseñar un problema y resolverlo usando búsqueda heurística (A*).


## 2 Descripción de los modelos

### 2.1 Problema 1

#### Representación del problema

Según el enunciado del primer problema, se nos pide modelar un parking con cantidad variable de plazas reservadas para vehículos eléctricos o con congelador.
Para resolver el problema en sí, se representan los siguientes conceptos:

- Parking: se modelará como una matriz de longitud "n" filas y "m" columnas. Dichos parámetros se elegirán por el usuario.
- Plaza: Una plaza del aparcamiento anterior estará representada por una tupla con sus coordenadas. Por ejemplo, para la plaza 1, se representará como "(0,0)" en el problema.
- Vehículo: Un vehículo quedará definido por una cadena formada por la concatenación de su número identificador, su tipo (TSU si es urgente y TNU si no) y si tiene congelador o no.

Las variables del problema indispensables para su resolución serán representadas por todos y cada uno de los vehículos que deseen ingresar al parking. Sus valores de dominio, serán así las posiciones asignables a dicho vehículo en el parking. Para este problema, se ha diferenciado entre dos tipos de variables:

- Vehículos que están equipados con congelador: Estos vehículos solo podrán ser asignados plazas eléctricas en el parking.
- Vehículos que no tienen sistemas eléctricos: Dicho tipo podrá tener asignada cualquier plaza en el parking. 

#### Modelado de Restricciones

La descripción de las restricciones dada la representación anterior del problema queda así:

- Solo puede haber un vehículo aparcado en una plaza del aparcamiento: Dado que la implementación utiliza una matriz de elementos únicos para cada casilla, esta restricción se cumple por defecto.
- No se pueden quedar vehículos sin plaza: Una variable "vehículo" solo puede quedar con un valor en su dominio. Habiendo definido las variables del problema como los vehículos y no las plazas, nos aseguramos de que todo coche esté exactamente en una plaza.
- Los vehículos provistos de congelador solo podrán asignarse a plazas eléctricas: Esta restricción también se contempla con la representación del problema.
- Un vehículo de tipo urgente o TSU, no puede tener aparcado uno no urgente o TNU en todas las posiciones de su derecha en su misma fila. Esto se realiza comparando las coordenadas "y" e igualando las "x" de cada par de posiciones en el parking. De esta manera, no se pondrá un TNU en posiciones mayores a un TSU en su misma fila.
- Para poder entrar y salir de su plaza, cada coche no puede tener aparcado uno delante y atrás a la vez. Así se asegura una maniobrabilidad decente en el parking. Esta restricción se satisface comparando todas las combinaciones de trios de posiciones a asignar para filtrar las que no estén consecutivas.
- De la restricción anterior, surge otra extra para los casos de los aparcamientos en los laterales o bordes del parking. Resulta que si un vehículo está aparcado en una de la primera fila del parking, no puede tener otro vehículo aparcado justo abajo ya que no podría salir. Lo mismo pasa con los vehículos que aparquen en la última fila del parking pero con las posiciones de arriba. Para satisfacer esta restricción, se buscan los pares de posiciones que estando una en el borde, la otra no sea consecutiva por arriba o por abajo.

### 2.2 Problema 2

#### Parámetros globales

Para resolver este problema, se han necesitado los siguientes parámetros globales (constantes):

- $P_T \in \N$: plazas totales de la ambulancia
- $P_{T_C} \in \N$: plazas reservadas para pacientes contagiosos
- $E_0 \in \N$: energía inicial de la ambulancia, y valor al que se reinicia al pasar por el parking
- $\text{Casillas} = \{1, 2, X, N, C, CN, CC, P\}$: conjunto de posibles contenidos de una casilla
- $M_{ij} \in \text{Casillas} \quad (i, j \in \N, i < N, j < M)$: mapa del problema, donde cada elemento indica el contenido de la casilla correspondiente
- $\operatorname{energía}: \text{Casillas} \rightarrow \N$: función que devuelve el coste de energía de pasar por una casilla
  $$\operatorname{energía}(c) = \begin{cases}
      2 & c = 2 \\
      1 & X \not = c \not = 2
  \end{cases}$$

#### Estado

Los posibles estados de la ambulancia se han representado con una tupla con los siguientes valores:

- $P_N \in \N$: número de plazas actualmente ocupadas por pacientes no contagiosos
- $P_C \in \N$: número de plazas actualmente ocupadas por pacientes contagiosos
- $E \in \N$: energía actual de la ambulancia
- $\text{Pos} \in \N^{2 \times 1}$: posición actual de la ambulancia
- $\text{Visitados}$: campo de bits que codifica los pacientes que han sido recogidos, donde la posición $i$ indica si el paciente con ID $i$ ha sido recogido o no. Este ID se obtiene a partir de posición del paciente. Esto se eligió para reducir la cantidad de memoria necesaria para codificar cada estado

Con esto, el estado inicial sería el siguiente:

- $P_N = P_C = 0$
- $E = E_0$
- $\text{Pos} = \text{posición del parking}$
- $\text{Visitados} = 0$

El estado final sería cualquier estado que cumpla las siguientes condiciones:

- $P_N = P_C = 0$
- $\text{Pos} = \text{posición del parking}$
- $\text{Visitados} = \text{campo de bits con todos los bits a }1$

#### Operadores

Este problema cuenta con un único operador: $\operatorname{move}(x, y)$. Este operador mueve la ambulancia según el desplazamiento $(x, y)$. Para cada estado, sus sucesores serán los resultantes de aplicar este operador con los desplazamientos $(-1, 0)$, $(1, 0)$, $(0, -1)$, y $(0, 1)$, los cuales se corresponden con los movimientos horizontales y verticales permitidos.

Las precondiciones son las siguientes:
\begin{align}
    0 \leq \text{Pos}_x + x < N \\
    0 \leq \text{Pos}_y + y < M \\
    M[\text{Pos} + (x, y)] \not = X \\
    E \geq \operatorname{energía}(M[\text{Pos} + (x, y)])
\end{align}

\begin{itemize}
\item[$(1)(2)$] La nueva posición ($\text{Pos} + (x, y)$) está dentro del mapa
\item[$(3)$] La nueva posición ($\text{Pos} + (x, y)$) se puede transitar
\item[$(4)$] La ambulancia tiene la suficiente energía para atravesar la casilla
\end{itemize}

Los efectos son los siguientes:

- Se copian los valores del estado antes de aplicar el operador
- $\text{Pos} = \text{Pos} + (x, y)$
- $E = E - \operatorname{energía}(M[\text{Pos} + (x, y)])$
- Si $M[\text{Pos} + (x, y)] = CN \Rightarrow P_N = 0$
- Si $M[\text{Pos} + (x, y)] = CC \Rightarrow P_C = 0$
- Si $M[\text{Pos} + (x, y)] = P \Rightarrow E = E_0$
- Si $M[\text{Pos} + (x, y)] = N$, el paciente se puede recoger ($P_C = 0, P_N \leq P_T$), y no está marcado en $\text{Visitados}$ $\Rightarrow P_N = P_N + 1$ y se marca el paciente en $\text{Visitados}$
- Si $M[\text{Pos} + (x, y)] = C$, el paciente se puede recoger ($P_C < P_{T_C}, P_N \leq P_T - P_{T_C}$), y no está marcado en $\text{Visitados}$, $\Rightarrow P_C = P_C + 1$ y se marca el paciente en $\text{Visitados}$

El coste del operador es $\operatorname{energía}(M[\text{Pos} + (x, y)])$

#### Heurísticas

Para las heurísticas definidas se ha usado la distancia de Manhattan para estimar la distancia entre dos posiciones dadas. Además, ambas heurísticas relajan las precondiciones $(3)$ y $(4)$, y las condiciones sobre cuando se puede recoger un paciente.

La primera heurística es el coste total de recoger al paciente no recogido más lejano (restringido a los pacientes contagiosos si la ambulancia ya tiene pacientes contagiosos), ir al centro de pacientes contagiosos (si es necesario), ir al centro de pacientes no contagiosos (si es necesario), y finalmente volver al parking.

Esta heurística es admisible porque estos pasos siempre se tendrán que hacer en orden al menos una vez al final. Además, las condiciones para ir a los centros de pacientes garantizan que el coste de sus respectivos pasos solo se añade si el paso realmente es necesario, garantizando que nunca se sobrestima el coste real.

La segunda heurística es una modificación de la primera. En el primer paso, si la ambulancia sí tiene algún paciente contagioso, además de recoger al paciente contagioso no recogido más lejano, también se añade el coste de ir al centro de pacientes contagiosos y recoger al paciente no contagioso no recogido más lejano. Además, en este caso no se volvería a ir al centro de pacientes contagiosos, asumiendo que ya se han entregado todos los pacientes contagiosos en el primer viaje.

Si la ambulancia tiene algún paciente contagioso, tendrá que pasar por el centro de pacientes contagiosos antes de recoger a algún otro paciente no contagioso. Por lo tanto, añadir en estos casos a la heurística original una estimación de este coste que siempre subestima el real no hace que deje de ser admisible. Además, como esta heurística es la primera con un sumando extra, está más informada.

## 3 Análisis de resultados

### 3.1 Problema 1

#### Resultado Obtenido

Según nuestra implementación en Python con la librería "Python-Constraints" de este problema, al ejecutarlo con un archivo de ejemplo (ejemplo de la memoria), este resulta obteniendo un valor de 2175288 posibles soluciones. 

Este dato es imposible de comprobar ya que se tardaría mucho tiempo en realizar las combinaciones de todas las casillas a mano. Para demostrar que las soluciones que arroja nuestro programa o script son las necesarias, se realizarán una serie de tests desarrollados en el lenguaje de __BASH SCRIPTING__. 

#### Definición de casos de prueba

A la hora de realización de los tests mencionados, se han tenido en cuenta los siguientes casos de uso para los diferentes inputs del programa:

1) Parking con mismo número de posiciones eléctricas que vehículos con congelador (Posiciones eléctricas consecutivas).
2) Parking con más plazas eléctricas de las necesarias. Entre las soluciones disponibles de este caso, se encontrarán algunas en las que un vehículo sin congelador por lo menos haya ocupado una plaza eléctrica.
3) Parking con mismo número de plazas eléctricas que de vehículos con congelador (Posiciones eléctricas no consecutivas).
4) Parking con grupos de tres plazas eléctricas consecutivas. La finalidad es testear la restricción que verifica la maniobrabilidad en el parking.
5) Parking con posiciones consecutivas en los bordes. Testeo de restricciones en las filas primera y última.

#### Planteamiento de tests

Los archivos generados como _"Inputs"_ de muestro script "_CSPParking.py_" se crean de la siguiente manera:

1. Para el primer caso de prueba, se definirá un parking de tamaño __2x1__. En dicho parking las dos casillas son eléctricas y solo entrarán al parking 2 vehículos con congelador. En este problema, la restricción de tipos se ve relajada debido a que no hace efecto en un parking de una columna. La restricción de maniobrabilidad también lo está debido a que se necesita al menos un trio para poder tener influencia. La restricción que se prueba entonces es la de consecutividad en los bordes. Es por eso que, al no poder salir los coches del parking fácilmente al haber aparcado,  no existen soluciones para este problema.

2. En el segundo caso de prueba, se creará un parking de tamaño __5x6__. En este, se habilitarán cuatro casillas eléctricas no consecutivas en los bordes de las treinta disponibles. También las ambulancias que necesitarán aparcar en dicho parking serán tres con congelador y de distintos tipos de urgencia. Las restricciones que tienen efecto en las soluciones son las de los bordes (en las soluciones, no saldrán las que tengan una casilla contigua a las eléctricas ocupadas), la de posiciones consecutivas (no puede haber un trio de posiciones consecutivas en ninguna solución) y la restricción de tipos (siempre los vehículos urgentes tendrán a los no urgentes detrás suya y no al revés).

3. Para el tercer caso de prueba, se construirá un parking de dimensiones __3x3__. Para este, las posiciones eléctricas serán las cuatro esquinas del cuadrado. Cuatro vehículos eléctricos querrían ocupar su plaza en el aparcamiento (uno de tipo urgente y tres no urgentes). Ya que las posiciones electricas no son consecutivas mutuamente y no hay más ambulancias que las eléctricas, la única restricción que tiene efecto sobre las soluciones es la de los tipos (urgente y no urgente). Entre las soluciones, se debería de observar que no existen soluciones que no tengan al vehículo urgente (vehículo con id de 1) en la esquina superior derecha o inferior derecha.

4. En este caso de prueba, se edificará un parking de tamaño __6x4__. Para probar la maniobrabilidad de las posiciones consecutivas, se habilitarán seis posiciones electricas organizadas en trios consecutivos (concretamente en la columna 2 y 4). Para este escenario, seis ambulancias eléctricas de tipo no importante van a ingresar al parking. Dado que todos los vehículos son eléctricos y obligatoriamente tienen que estar en plazas electricas consecutivas, el algoritmo no dará soluciones válidas ya que también se restringen las posiciones consecutivas por otro lado.

5. Para el último caso de prueba, se definirá un aparcamiento de tamaño __3x3__. En él, no habrá trios de posiciones consecutivas pero si posiciones electricas consecutivas en los bordes de dicho cuadrado (seis posiciones). Todos los seis vehículos que desean aparcar son eléctricos y forzosamente deben estar en posiciones electricas, que a la vez están consecutivas, luego el algoritmo de resolución no dará soluciones válidas al igual que en el anterior caso de prueba.

#### Comprobación de soluciones

Para el método de comprobación de las soluciones obtenidas, los resultados siguientes son los esperados:

1) Los casos de prueba 1, 4 y 5 no deben tener soluciones válidas para los archivos generados.

2) El caso de prueba 2 debería dar $4! - 4*2 = 16$ soluciones. Esto se puede comprobar ya que 4! soluciones son las posibles combinaciones de 4 elementos en 4 plazas de parking. Dado que hay dos vehículos que son de tipo urgente, cada uno resta al total de soluciones 4 que no se pueden dar debido a la restricción de tipos.

3) El caso de prueba 3 debería arrojarnos $\frac{4!}{2}$ soluciones debido a que hay 4! posibles maneras de permutar 4 vehículos en 4 posibles plazas del mismo. Como la mitad de las soluciones no respetan la restricción de tipo dejando a la ambulancia urgente en las esquinas superior izquierda e inferior izquierda, dichas soluciones se descartan de las finales (cumpliendo así todas las restricciones).

#### Conclusiones de los tests

Dado que los resultados obtenidos por los tests son los esperados, los tests se han ejecutado correctamente, probando así que nuestro algoritmo es legítimo en varios casos base.

### 3.2 Problema 2

#### Resultado obtenido

Para el problema dado, el programa encuentra 2 soluciones óptimas con coste 88 función de la heurística usada. Para la primera heurística tiene una longitud del plan de 85 pasos, mientras que para la segunda tiene una longitud de 83 pasos

#### Casos de prueba

Los casos de prueba implementados son los siguientes:

1) Mapa lineal con un único paciente contagioso entre el parking y el centro de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos y volver
2) Mapa lineal con un único paciente no contagioso entre el parking y el centro de pacientes contagiosos. La solución óptima será ir al centro de pacientes no contagiosos y volver
3) Mapa lineal con un paciente no contagioso seguido de un paciente contagioso, seguido del centro de pacientes contagiosos y de pacientes no contagiosos. La solución óptima será ir al centro de pacientes no contagiosos y volver
4) Mapa lineal con un paciente no contagioso seguido de un paciente contagioso, seguido del centro de pacientes no contagiosos y de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos y volver, y se entregará al paciente no contagioso a la vuelta
5) Mapa lineal con un paciente contagioso seguido de un paciente no contagioso, seguido del centro de pacientes no contagiosos y de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos, volver a por el paciente no contagioso, ir al centro de pacientes no contagiosos, y volver al parking
6) Mapa lineal con un paciente contagioso seguido de un paciente no contagioso, seguido del centro de pacientes contagiosos y de pacientes no contagiosos. La solución óptima será ir al centro de pacientes contagiosos, volver a por el paciente no contagioso, ir al centro de pacientes no contagiosos, y volver al parking
7) Mapa en el que los pacientes/centros de pacientes están separados del parking por una casilla no transitable. No hay solución
8) Mapa con un paciente no contagioso seguido de 2 caminos con diferente longitud al centro de pacientes no contagiosos. La solución óptima será ir y volver del centro por el camino más corto
9) Mapa lineal con un único paciente no contagioso entre el parking y su centro, en el cual hay la energía justa para ir a su centro y volver. La solución óptima es ir al centro del paciente y volver
10) Mapa lineal en el cual el parking está entre un paciente no contagioso y su centro, y solo hay energía suficiente para llegar al centro desde el paciente si se recarga en el parking. La solución óptima es recoger al paciente, entregarlo en su centro, y volver al parking
11) Mapa con un único paciente no contagioso en el cual hay energía suficiente para ir y volver desde el parking al paciente o a su centro, pero no para realizar ambos en un único trayecto. La solución óptima es recoger al paciente no contagioso, volver al parking, ir a su centro, y volver al parking
12) Mapa lineal con más pacientes no contagiosos de los que puede llevar la ambulancia entre el parking y su centro. La solución óptima es ir a su centro, volver a por los restantes, volver a ir a su centro, y volver al parking
13) Mapa lineal con suficientes pacientes no contagiosos como para tener que usar las plazas de pacientes contagiosos pero no como para llenar la ambulancia, seguidos de un paciente contagioso y los centros de los pacientes. La solución óptima es ir al centro de pacientes no contagiosos, volver a por el paciente contagioso, ir al centro de pacientes contagiosos, y volver al parking
14) Mapa lineal con más pacientes contagiosos de los que puede llevar la ambulancia entre el parking y su centro. La solución óptima es ir a su centro, volver a por los restantes, volver a ir a su centro, y volver al parking

Tras ejecutar el programa con todos los casos de prueba, este obtiene siempre la solución óptima si existe

#### Rendimiento

Debido a la similitud de las heurísticas, en muchos de los casos de prueba el resultado es el mismo. Sin embargo, en los casos en los que la modificación de la segunda heurística se usa, esta es capaz de expandir significativamente menos nodos.

Para un caso complejo como el problema dado, la primera heurística necesita expandir $\sim 174$ millones de estados, mientras que la segunda es capaz de expandir únicamente $\sim 87$ millones

## 4 Conclusión
