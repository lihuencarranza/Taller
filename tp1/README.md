# Taller

## Compilación

### Para correr
```cargo run "expression" "path"``` 
En este caso incluí archivos en este mismo directorio, se lo puede invocar con "texto1.txt"

### Para testear
```cargo test``` 


## ¿Qué se supone que funciona?

Las Regex que contengan tipo de repetición Exact con todos los tipos de valores (Literal, Wildcard, Class, Vowel, OneOf)

## ¿Qué hay que cambiar?

- Cargo clippy
- Documentación
- Los otros tipos de repeticiones (Any, None, Range)
- Modularización del matcheo de la regex con las palabras disponibles
- Testear el main
- Eliminar clones y unwrap del main


