//un struct es un tipo de dato custom que agrupa distintos valores relacionados y les asigna un nombre, de acuerdo a un sentido.
//son equivalentes a un objeto y sus atributos en los lenguajes orientados a objetos.

//los structs son similares a las tuples, con la ventaja de que en los structs los valores deben son nombrados
//mientras que en las tuples se confia en el orden en que se asignan (index) o deconstruyen
//esas parejas de clave-valor en los structs se conocen como campos
//son plantillas para crear instancias

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn main() {
    //cuando se crea una instancia, no importa el orden de asignacion de los campos
    //por que se indica la etiqueta

    let usr1 = User {
        email: String::from("rusty@gmail.com"),
        username: String::from("rusty_dev"),
        active: true,
        sing_in_count: 1,
    };

    //una instancia tambien puede ser mutable, lo que permite modificar los campos
    //no se pueden marcar solo algunos campos como mutables, solo toda la instancia

    let mut usr2 = User {
        email: String::from("rusty@gmail.com"),
        username: String::from("rusty_dev"),
        active: true,
        sing_in_count: 1,
    };

    //para acceder a los campos se usa instancia.campo

    usr2.email = String::from("rustaceans@gmail.com");
    let cool_username = usr1.username;

    let user3 = build_user(String::from("dummy_email"), String::from("dummy_username"));

    //crear instancias a partir otras instancias de forma manual
    
    //como toma campos de user3 que se pasan con move (username), user3 pierde su ownership
    //si solo copiara active o sing_in_count, s3 no perderia su ownership, por que son de tipos que implementan copy
    let user4 = User {
        email: String::from("another_email@gmail.com"),
        username: user3.username,
        active: user3.active,
        sing_in_count: user3.sing_in_count,
    };

    //crear instancias a partir de otras instancias con struct update syntax
    //el unico valor que se va a cambiar para user5 es sing_in_count, por lo que este se asigna explicitamente
    // con la sintaxis de actualizacion indicamos que los demas valores seran los de user4
    let user5 = User {
        sing_in_count: 2,
        ..user4 //struct update syntax
    };

    //println!("usr4 username: {}", user4.username); //no compila por que usr4 se movio al copiar sus campos a usr5

    //struct tuple

    //son plantillas de tuplas, es util cuando nombrar los campos resulta poco practico

    struct Color (u32, u32, u32, u32); //util para crear instancias que representan un valor en rgba
    struct Point (u32,u32,u32); // cordenadas de un punto

    let black = Color(0,0,0,1);
    let a_point = Point(1,2,3);

    //structs como unidad
    //Son usados para implementar un trait, pero no hay datos que se quieran almacenar en forma de campos

    struct alwaysEqual;

    let subject = alwaysEqual; // cuando el struct es unidad, no se requieren parentesis o curly braces

    //Hasta este punto solo hemos usado tipos owner para los campos
    //Se pueden usar referencias (incluidos slices) pero se deben especificar lifetimes

    //no compila por que falta especificar el lifetime de los campos
    //struct Student {
        //nombre: &str,
        //plantel: &str,
    //}
}

// shorthand de inicializacion de campos
//alternativamente podemos escribir la asignacion de cada campo con el parametro correspondiente
//para usar el shorthand el parametro y el campo deben tener el mismo nombre
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}