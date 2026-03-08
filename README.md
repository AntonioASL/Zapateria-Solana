ShoeLedger — Inventario de Zapatos en Solana

ShoeLedger es un programa on-chain construido en Rust utilizando el framework Anchor Framework sobre la blockchain de Solana.

Su objetivo es permitir que una zapatería o negocio de calzado registre y administre su inventario de zapatos de forma descentralizada, segura y permanente en la blockchain.

Este sistema guarda la información directamente en cuentas del programa dentro de la red de Solana, lo que significa que los datos no dependen de un servidor tradicional ni de una base de datos centralizada.

Objetivo del proyecto

ShoeLedger funciona como un sistema de gestión de inventario de zapatos en blockchain, que permite realizar operaciones básicas sobre los productos registrados en la zapatería.

Las principales funciones del sistema son:

• Crear el perfil de una zapatería asociado a una wallet
• Registrar nuevos zapatos con su información
• Consultar los zapatos registrados en el inventario
• Activar o desactivar la disponibilidad de un zapato
• Eliminar zapatos del sistema

Toda la información queda registrada de manera transparente, segura e inmutable dentro de la red de Solana.

Estructura del programa

El programa organiza los datos dentro de la blockchain en una estructura jerárquica simple.

Wallet (Owner)
│
└── Cuenta de Zapatería
     │
     ├── Zapato 1
     ├── Zapato 2
     └── Zapato 3

Cada zapatería tiene su propio inventario de zapatos vinculado a la wallet del propietario.

Esto permite que cada usuario controle su propia tienda dentro de la blockchain.

Estructuras de datos principales
Zapatería
Campo	Tipo	Descripción
owner	Pubkey	Dirección de la wallet propietaria
nombre	String	Nombre de la zapatería
zapatos	Vec	Lista de zapatos almacenados
Zapato
Campo	Tipo	Descripción
nombre	String	Nombre o modelo del zapato
talla	u8	Talla del zapato
precio	u16	Precio del zapato
disponible	bool	Indica si el zapato está disponible en el inventario
Funciones del programa

El contrato inteligente incluye varias instrucciones para interactuar con los datos almacenados.

crear_zapateria(nombre)

Crea una nueva cuenta de zapatería vinculada a la wallet del propietario.

agregar_zapato(nombre, talla, precio)

Agrega un nuevo zapato al inventario de la zapatería.

ver_zapatos()

Muestra la lista completa de zapatos registrados en la zapatería.

alternar_estado(nombre)

Cambia el estado de disponibilidad de un zapato, permitiendo marcarlo como disponible o no disponible.

eliminar_zapato(nombre)

Elimina un zapato específico del inventario de la zapatería.

Direcciones derivadas (PDA)

El programa utiliza Program Derived Addresses (PDA) para generar cuentas únicas dentro de la red de Solana.

Cuenta	Seeds utilizadas
Zapatería	["zapateria", owner_pubkey]

Esto asegura que:

• Cada wallet puede tener solo una zapatería registrada
• Los datos almacenados no pueden ser modificados por terceros
• Solo el propietario de la cuenta puede actualizar su inventario

Cómo ejecutar el proyecto

Para probar el programa puedes utilizar Solana Playground.

Pasos básicos:

Abrir Solana Playground

Crear o pegar el código dentro del archivo src/lib.rs

Conectar tu wallet en la red Devnet

Presionar Build para compilar el programa

Presionar Deploy para publicarlo

Ejecutar las funciones desde el panel de pruebas

Ejemplo de uso

Flujo típico de interacción con el programa:

crear_zapateria("Zapatería Central")

agregar_zapato("Nike Air Max", 27, 1500)

agregar_zapato("Adidas Run Falcon", 26, 1200)

alternar_estado("Nike Air Max")

eliminar_zapato("Adidas Run Falcon")

Esto permite gestionar el inventario de la zapatería directamente en la blockchain.

Tecnologías utilizadas
Tecnología	Uso dentro del proyecto
Solana	Blockchain donde se ejecuta el programa
Anchor	Framework para desarrollar smart contracts
Rust	Lenguaje principal del programa
TypeScript	Cliente para interactuar con el programa
Autor

Proyecto desarrollado como parte del aprendizaje y práctica de desarrollo de smart contracts en Solana, implementando un sistema descentralizado para la gestión de inventario de una zapatería utilizando tecnología blockchain.
