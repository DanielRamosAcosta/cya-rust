# Práctica 10: Simulación de una Máquina de Turing

> Semana del 17 al 21 de noviembre

## Objetivo

El objetivo de esta práctica es diseñar en C++ un programa simulador de máquinas de Turing. El programa deberá ser capaz de leer la tabla de transición de una máquina de Turing así como el contenido inicial de la cinta de entrada de la máquina. Tras obtener esta información, el programa deberá simular el comportamiento de cada una de las acciones a llevar a cabo por la máquina. Cabe destacar que las máquinas de Turing a simular tendrán una única cinta y una única pista. Para verificar el comportamiento del programa desarrollado se hará uso de la herramienta JFLAP (disponible en http://jflap.org y en el aula virtual de la asignatura).

## Especificación de la Máquina de Turing

La máquina de Turing a simular vendrá especificada en un fichero cuya extensión será `.tm` _(Turing Machine)_. Todo fichero `.tm` contiene la especificación de una tabla de transición de una determinada máquina de Turing. El formato del fichero será el siguiente:

- Línea 1: entero indicando el número de estados
- Línea 2: entero indicando el primer estado final (todos los estados con identificador igual o mayor al primer estado final se consideran estados finales)
- Línea 3: entero indicando el número de t-uplas que componen la máquina
- Líneas sucesivas: una t-upla en cada línea. El formato de las t-uplas es el siguiente: `estado$entrada$escribe$movimiento$siguiente_estado`
  - \$: espacio en blanco
  - estado: entero (identificador) que representa el estado
  - entrada: símbolo del alfabeto de entrada
  - escribe: símbolo del alfabeto de la cinta a escribir en la cinta para dicho estado y entrada
  - movimiento: movimiento a realizar por la cabeza de lectura/escritura de la cinta:
    - L: izquierda
    - R: derecha
    - S: parar
  - siguiente estado: siguiente estado al que transita la máquina

Los símbolos del alfabeto podrán ser dígitos o letras minúsculas/mayúsculas, aunque en cualquier caso, el simulador deberá codificarlos como enteros. Los estados serán números enteros que comenzarán con el estado inicial igual a cero. El símbolo blanco, se representará por el símbolo `$`.

## Funcionamiento general del programa

El programa principal debería ofrecer al usuario al menos las siguientes opciones:

* **Leer máquina de Turing**: al seleccionar esta opción se deberá solicitar al usuario que introduzca el nombre del fichero `.tm` donde se encuentra la especificación de la máquina. A continuación, se deberá crear la máquina de Turing a partir de la especificación dada en el fichero. Habrá que notificar al usuario si se produce algún error en la creación de la máquina.
* **Mostrar máquina de Turing**: al seleccionar esta opción se mostrará por pantalla la máquina de Turing actualmente definida (previamente leída de fichero) en nuestro programa. Para mostrar la máquina de Turing por pantalla se seguirá el formato establecido para los ficheros `.tm`.
* **Simular máquina de Turing**: al seleccionar esta opción se deberá solicitar al usuario que introduzca una cadena. Para la cadena indicada por el usuario se deberá determinar si es aceptada o no por la máquina de Turing actualmente definida. Además, se ofrecerá una opción para que el usuario pueda ver, paso a paso, los movimientos que va haciendo la máquina hasta que acepta/rechaza la cadena. Para iniciar la cinta de la máquina de Turing hay que tener en cuenta que a la izquierda y a la derecha de este contenido inicial tendremos símbolos blancos. Al igual que en el caso anterior, esta opción tampoco se podrá ejecutar hasta que se haya definido una máquina de Turing.

## Pruebas

A modo de ejemplo, y para chequear el funcionamiento del simulador se recomiendan hacer las pruebas descritas a continuación.

1. Partiendo de la práctica anterior, definir el fichero `.tm` de la máquina de Turing que reconoce el lenguaje `L = {0^n1^n | n ≥ 1}` y simular la máquina para una cadena que pertenezca al lenguaje y para otra cadena que no pertenezca al lenguaje.
2. Realizar pruebas similares para la máquina de Turing que reconoce el lenguaje `L = {w | la longitud de w es par}`, y que también fue diseñada en la práctica anterior.
