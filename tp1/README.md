# Taller

## Compilación

### Para correr
```cargo run "expression" "path"``` 
En este caso incluí archivos en este mismo directorio, se lo puede invocar con "texto1.txt"

### Para testear
```cargo test``` 


## ¿Qué se supone que funciona?

Las Regex que contengan tipo de repetición Exact con todos los tipos de valores (Literal, Wildcard, Class, OneOf)

## ¿Qué hay que cambiar?

- Cargo clippy
- Documentación
- Los otros tipos de repeticiones (Any, None, Range)
- Modularización del matcheo de la regex con las palabras disponibles
- Testear el main
- Eliminar clones y unwrap del main
- Estoy pensando en cambiar la logica de None y convertirlo a NonOf dentro de RegexVal
- No me gustó como implementé el '|'. Dentro de la Regex, está como un literal, cuando hace el matcheo, crea una nueva regex si encuentra el '|'

