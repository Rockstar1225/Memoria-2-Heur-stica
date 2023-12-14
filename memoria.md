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

- Modelar un problema y resolverlo utilizando SCP.
- Diseñar un problema y resolverlo usando búsqueda heurística (A*).

## 2 Descripción de los modelos

### 2.1 Problema 1

#### Representación del problema

Según el enunciado del primer problema, se nos pide modelar un parking con cantidad variable de plazas reservadas para vehículos eléctricos o con congelador.
Para resolver el problema en sí, se representan los siguientes conceptos:

- Parking: se modelará como una matriz de $n$ filas y $m$ columnas. Dichos parámetros se elegirán por el usuario.
- $\text{Plaza} \in \N^2$: Una plaza del aparcamiento anterior estará representada por una tupla con sus coordenadas. Por ejemplo, para la plaza en la primera fila y columna, se representará como $(1, 1)$ en el problema.
- Vehículo: Un vehículo quedará definido por una cadena formada por la concatenación de su número identificador, su tipo (TSU si es urgente y TNU si no) y si tiene congelador o no. Ej. "1-TSU-X".

Las variables del problema indispensables para su resolución serán representadas por todos y cada uno de los vehículos que deseen ingresar al parking ($V$). Sus dominios serán el conjunto de posiciones asignables a dicho vehículo en el parking. Para este problema, se ha diferenciado entre dos tipos de variables:

- Vehículos equipados con congelador: Estos vehículos solo podrán ser asignados plazas eléctricas en el parking.
- Vehículos sin congelador: Estos vehículos podrán tener asignada cualquier plaza en el parking.

De esta forma, se garantiza que se cumplen las restricciones sobre asignar a todos los vehículos a una plaza del parking, y que los vehículos con congelador tengan asignada una plaza eléctrica del parking.

#### Modelado de Restricciones

Para definir las restricciones, se ha dividido el conjunto de vehículos ($V$) en 2 subconjuntos disjuntos:

- $V_U$: conjunto de vehículos urgentes (TSU).
- $V_N$: conjunto de vehículos no urgentes (TNU).

La descripción de las restricciones dada la representación anterior del problema queda así:
\begin{align}
    a \not = b & \quad \forall a, b \in V \\
    (a_y \not = b_y) \lor (a_x > b_x) & \quad \forall a \in V_U, b \in V_N \\
    \lnot (a_x = b_x = c_x) \lor \min(a_y, b_y, c_y) + 2 \not = \max(a_y, b_y, c_y) & \quad \forall a, b, c \in V, a \not = b \not = c, a \not = c \\
    a_x \not = b_x \lor \min(a_y, b_y) + 1 \not = \max(a_y, b_y) \lor 1 \not = \min(a_y, b_y) \not = n - 1 & \quad \forall a, b \in V, a \not = b \\
    n \geq 2 \lor \|V\| = 0 &
\end{align}

(1) Todos los vehículos se tienen que aparcar en plazas distintas del aparcamiento.
(2) Un vehículo de tipo urgente (TSU) no puede tener aparcado uno no urgente (TNU) en todas las posiciones de su derecha en su misma fila.
(3) Ningún trio de vehículos puede ocupar 3 plazas consecutivas en una columna, ya que esto no cumpliría con la restricción de maniobrabilidad para el vehículo central.
(4) Igual a la restricción $(3)$, pero aplicado al caso de un vehículo aparcado en el bordes del parking. En este caso, un vehículo aparcado en el borde inferior o superior del parking no puede tener a otro aparcado en el lado opuesto al borde.
(5) Restricción $(4)$ aplicada al caso de un parking con una única fila. Ninguna de las plazas del parking tiene un hueco libre arriba o abajo, luego el problema no tiene solución en este caso.

### 2.2 Problema 2

#### Parámetros globales

Para resolver este problema, se han necesitado los siguientes parámetros globales (constantes):

- $P_T \in \N$: plazas totales de la ambulancia.
- $P_{T_C} \in \N$: plazas reservadas para pacientes contagiosos.
- $E_0 \in \N$: energía inicial de la ambulancia, y valor al que se reinicia al pasar por el parking.
- $\text{Casillas} = \{1, 2, X, N, C, CN, CC, P\}$: conjunto de posibles contenidos de una casilla.
- $M_{ij} \in \text{Casillas} \quad (i, j \in \N, i < N, j < M)$: mapa del problema, donde cada elemento indica el contenido de la casilla correspondiente.
- $\operatorname{energía}: \text{Casillas} \rightarrow \N$: función que devuelve el coste de energía de pasar por una casilla.
  $$\operatorname{energía}(c) = \begin{cases}
      2 & c = 2 \\
      1 & X \not = c \not = 2
  \end{cases}$$

#### Estado

Los posibles estados de la ambulancia se han representado con una tupla con los siguientes valores:

- $P_N \in \N$: número de plazas actualmente ocupadas por pacientes no contagiosos.
- $P_C \in \N$: número de plazas actualmente ocupadas por pacientes contagiosos.
- $E \in \N$: energía actual de la ambulancia.
- $\text{Pos} \in \N^{2 \times 1}$: posición actual de la ambulancia.
- $\text{Visitados}$: campo de bits que codifica los pacientes que han sido recogidos, donde la posición $i$ indica si el paciente con ID $i$ ha sido recogido o no. Este ID se obtiene a partir de posición del paciente. Esto se eligió para reducir la cantidad de memoria necesaria para codificar cada estado.

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

Este problema cuenta con un operador: $\operatorname{move}(x, y)$. Este operador mueve la ambulancia según el desplazamiento $(x, y)$. Para cada estado, sus sucesores serán los resultantes de aplicar este operador con los desplazamientos $(-1, 0)$, $(1, 0)$, $(0, -1)$, y $(0, 1)$, los cuales se corresponden con los movimientos horizontales y verticales permitidos.

Las precondiciones son las siguientes:
\setcounter{equation}{0}
\begin{align}
    0 \leq \text{Pos}_x + x < N \\
    0 \leq \text{Pos}_y + y < M \\
    M[\text{Pos} + (x, y)] \not = X \\
    E \geq \operatorname{energía}(M[\text{Pos} + (x, y)])
\end{align}

\begin{itemize}
\item[$(1)(2)$] La nueva posición ($\text{Pos} + (x, y)$) está dentro del mapa.
\item[$(3)$] La nueva posición ($\text{Pos} + (x, y)$) se puede transitar.
\item[$(4)$] La ambulancia tiene la suficiente energía para atravesar la casilla.
\end{itemize}

Los efectos son los siguientes:

- Se copian los valores del estado antes de aplicar el operador.
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

\clearpage

## 3 Análisis de resultados

### 3.1 Problema 1

#### Resultado Obtenido

Según nuestra implementación en `Python` con la librería `Python-Constraints` de este problema, al ejecutarlo con un archivo de ejemplo (ejemplo del enunciado), se obtienen $2175288$ posibles soluciones.

Este dato es imposible de comprobar ya que se tardaría mucho tiempo en realizar las combinaciones de todas las casillas a mano. Para demostrar que las soluciones que arroja nuestro programa son las necesarias, se realizarán una serie de tests desarrollados en `bash`.

#### Casos de prueba

Los casos de prueba implementados son los siguientes:

1) Parking de **1x2** con una única ambulancia. No hay solución ya que no se puede cumplir la restricción $(5)$
2) Parking de **2x1** con 2 vehículos eléctricos y 2 plazas eléctricas en la misma columna. No hay solución ya que no se puede cumplir la restricción $(4)$
3) Parking de **3x1** con 2 vehículos eléctricos y 2 plazas eléctricas en la misma columna al lado del borde superior del parking. No hay solución ya que no se puede cumplir la restricción $(4)$
4) Parking de **3x1** con 2 vehículos eléctricos y 2 plazas eléctricas en la misma columna al lado del borde inferior del parking. No hay solución ya que no se puede cumplir la restricción $(4)$
5) Parking de **4x1** con 3 vehículos eléctricos y 3 plazas eléctricas seguidas en la misma columna. No hay solución ya que no se puede cumplir la restricción $(3)$
6) Parking de **3x1** con 4 vehículos. No hay solución ya que no se puede cumplir la restricción $(1)$
7) Parking de **3x1** filas con 3 vehículos eléctricos y 2 plazas eléctricas. No hay solución ya que no se puede asignar a cada vehículo eléctrico una plaza eléctrica distinta.
8) Parking de **2x2** con un vehículo eléctrico TSU, un vehículo no eléctrico TNU, y 1 plaza eléctrica en la primera columna. Hay una única solución en la que el vehículo TNU está en la esquina opuesta al TSU debido a la restricción $(2)$
9) Parking de **2x2** con un vehículo eléctrico TSU, un vehículo no eléctrico TSU, y 1 plaza eléctrica en la primera columna. Hay 2 soluciones en la cual el vehículo eléctrico está en la plaza eléctrica y el no eléctrico está en la segunda columna debido a la restricción $(4)$
10) Parking de **2x2** con un vehículo eléctrico TSU, un vehículo no eléctrico TNU, y 1 plaza eléctrica en la segunda columna. Hay 2 soluciones en las cuales el vehículo eléctrico está en la plaza eléctrica y el no eléctrico está en la primera columna debido a la restricción $(4)$
11) Parking de **3x3** con 2 vehículos eléctricos TNU y 3 plazas eléctricas en la diagonal principal. Hay $6$ soluciones correspondientes a cada asignación de los vehículos a las plazas eléctricas.
12) Parking de **2x2** con 2 vehículos no eléctricos TNU y 2 plazas eléctricas en la diagonal principal. Hay $8$ soluciones correspondientes a cada asignación de vehículos a las plazas con los vehículos en columnas distintas.
13) Parking de **3x2** con 1 vehículo eléctrico TSU, 2 vehículos no eléctricos TNU, y 2 plazas eléctricas en primera columna. Hay $6$ soluciones en total: $2$ correspondientes al vehículo eléctrico en la plaza eléctrica central y los vehículos no eléctricos en las esquinas de la segunda columna, y $4$ correspondientes al vehículo eléctrico en la plaza eléctrica del borde, y uno de los vehículos no eléctricos en el borde opuesto de la primera columna y el otro en una de las $2$ plazas de la segunda columna cuya fila es distinta a la del vehículo eléctrico.

Tras ejecutar el programa con todos los casos de prueba, este obtiene siempre las soluciones esperadas, luego el programa es correcto.

### 3.2 Problema 2

#### Resultado obtenido

Para el problema dado, el programa encuentra 2 soluciones óptimas con coste 88 función de la heurística usada. Para la primera heurística tiene una longitud del plan de 85 pasos, mientras que para la segunda tiene una longitud de 83 pasos.

#### Casos de prueba

Los casos de prueba implementados son los siguientes:

1) Mapa lineal con un único paciente contagioso entre el parking y el centro de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos y volver.
2) Mapa lineal con un único paciente no contagioso entre el parking y el centro de pacientes contagiosos. La solución óptima será ir al centro de pacientes no contagiosos y volver.
3) Mapa lineal con un paciente no contagioso seguido de un paciente contagioso, seguido del centro de pacientes contagiosos y de pacientes no contagiosos. La solución óptima será ir al centro de pacientes no contagiosos y volver.
4) Mapa lineal con un paciente no contagioso seguido de un paciente contagioso, seguido del centro de pacientes no contagiosos y de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos y volver, y se entregará al paciente no contagioso a la vuelta.
5) Mapa lineal con un paciente contagioso seguido de un paciente no contagioso, seguido del centro de pacientes no contagiosos y de pacientes contagiosos. La solución óptima será ir al centro de pacientes contagiosos, volver a por el paciente no contagioso, ir al centro de pacientes no contagiosos, y volver al parking.
6) Mapa lineal con un paciente contagioso seguido de un paciente no contagioso, seguido del centro de pacientes contagiosos y de pacientes no contagiosos. La solución óptima será ir al centro de pacientes contagiosos, volver a por el paciente no contagioso, ir al centro de pacientes no contagiosos, y volver al parking.
7) Mapa en el que los pacientes/centros de pacientes están separados del parking por una casilla no transitable. No hay solución.
8) Mapa con un paciente no contagioso seguido de 2 caminos con diferente longitud al centro de pacientes no contagiosos. La solución óptima será ir y volver del centro por el camino más corto.
9) Mapa lineal con un único paciente no contagioso entre el parking y su centro, en el cual hay la energía justa para ir a su centro y volver. La solución óptima es ir al centro del paciente y volver.
10) Mapa lineal en el cual el parking está entre un paciente no contagioso y su centro, y solo hay energía suficiente para llegar al centro desde el paciente si se recarga en el parking. La solución óptima es recoger al paciente, entregarlo en su centro, y volver al parking.
11) Mapa con un único paciente no contagioso en el cual hay energía suficiente para ir y volver desde el parking al paciente o a su centro, pero no para realizar ambos en un único trayecto. La solución óptima es recoger al paciente no contagioso, volver al parking, ir a su centro, y volver al parking.
12) Mapa lineal con más pacientes no contagiosos de los que puede llevar la ambulancia entre el parking y su centro. La solución óptima es ir a su centro, volver a por los restantes, volver a ir a su centro, y volver al parking.
13) Mapa lineal con suficientes pacientes no contagiosos como para tener que usar las plazas de pacientes contagiosos pero no como para llenar la ambulancia, seguidos de un paciente contagioso y los centros de los pacientes. La solución óptima es ir al centro de pacientes no contagiosos, volver a por el paciente contagioso, ir al centro de pacientes contagiosos, y volver al parking.
14) Mapa lineal con más pacientes contagiosos de los que puede llevar la ambulancia entre el parking y su centro. La solución óptima es ir a su centro, volver a por los restantes, volver a ir a su centro, y volver al parking.

Tras ejecutar el programa con todos los casos de prueba, este obtiene siempre la solución óptima si existe, luego el programa es correcto.

#### Rendimiento

Debido a la similitud de las heurísticas, en muchos de los casos de prueba el resultado es el mismo. Sin embargo, en los casos en los que la modificación de la segunda heurística se usa, esta es capaz de expandir significativamente menos nodos.

Para un caso complejo como el problema dado, la primera heurística necesita expandir $\sim 174$ millones de estados, mientras que la segunda es capaz de expandir únicamente $\sim 87$ millones.

\clearpage

## 4 Conclusión

Para esta práctica, hemos aprendido los diferentes conceptos:

- Representar problemas en `Python`.
- Modelado de problemas reales.
- Representado de las restricciones del problema (funciones lambda).
- Resolver problemas de satisfacción de restricciones con la librería `Python-Constraints`.
- Extraer información de archivos.
- Volcar resultados de algoritmos en archivos de salida.
- Implementación de $A^*$ en `Rust`.
- Comparación de distintas heurísticas admisibles y sus efectos en la resolución del problema.
- Resolución de problemas reales con algoritmos de búsqueda.

Se puede concluir que en el tiempo de realización de la práctica, se ha seguido una curva de aprendizaje bastante razonable y se han aprovechado conceptos del lenguaje `Python` adquiridos en cursos anteriores.
