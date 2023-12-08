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

## 2 Descripción de los modelos

### 2.1 Problema 1

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

La primera heurística es el coste total de recoger al paciente no recogido más lejano, ir al centro de pacientes contagiosos (si se tienen pacientes contagiosos, o había pacientes contagios por recoger), ir al centro de pacientes no contagiosos (si se tienen pacientes no contagiosos, o había pacientes no contagiosos por recoger), y finalmente volver al parking.

Esta heurística es admisible porque estos pasos siempre se tendrán que hacer en orden al menos una vez cuando quede un paciente por recoger. Además, las condiciones para ir a los centros de pacientes garantizan que el coste de sus respectivos pasos solo se añade si el paso realmente es necesario, garantizando que nunca se sobrestima el coste real.

La segunda heurística es igual que la primera en todos los pasos menos el primero. Este primer paso solo se realiza si la ambulancia no tiene ningún paciente contagioso. En caso de tener alguno, en este paso se recoge al paciente contagioso no recogido más lejano, va al centro de pacientes contagiosos, y recoge al paciente no contagioso más lejano. Además, en este caso el segundo paso no se realiza, suponiendo que ya se han entregado todos los pacientes contagiosos en el primer viaje.

Esta heurística es admisible ya que es igual a la anterior si la ambulancia no tiene pacientes contagiosos. En caso de sí tenerlos, el problema especifica que no se pueden recoger más pacientes no contagiosos hasta ir al centro de pacientes contagiosos, luego esta modificación también subestimará el coste real en todos los casos. Además, este cálculo siempre obtiene un resultado mayor o igual que el primer paso original, luego esta heurística es más informada que la primera.

## 3 Análisis de resultados

### 3.1 Problema 1

### 3.2 Problema 2

## 4 Conclusión
