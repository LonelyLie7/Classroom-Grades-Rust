# Classroom Grades Exporter

Una herramienta de línea de comandos en Rust que se conecta a la API de Google Classroom para descargar información de cursos y exportar calificaciones a archivos CSV.

## ✨ Características

- Conecta a la API de Google Classroom (solo para cuentas docentes)
- Descarga información específica de cursos impartidos:
  - Título del curso
  - Lista de estudiantes
  - Asignaciones y entregas
- Exporta calificaciones a archivos CSV de manera concurrente
- Formato CSV: filas = estudiantes, columnas = asignaciones

## 📋 Prerrequisitos

1. **Rust y Cargo** instalados (versión 1.70 o superior recomendada)
2. Una cuenta de Google con permisos de docente
3. Un proyecto configurado en Google Cloud Console

## 🔧 Configuración

### 1. Configurar Google Cloud Console

1. Ve a [Google Cloud Console](https://console.cloud.google.com/)
2. Crea un nuevo proyecto o selecciona uno existente
3. Activa la API de Google Classroom:
   - Navega a "APIs y Servicios" > "Biblioteca"
   - Busca "Google Classroom API" y actívala
4. Configura la pantalla de consentimiento de OAuth:
   - En "APIs y Servicios" > "Pantalla de consentimiento de OAuth"
   - Selecciona "Externo" (para desarrollo) o "Interno" (para GSuite)
   - Completa la información requerida
   - Añade los scopes necesarios:
     - `https://www.googleapis.com/auth/classroom.courses.readonly`
     - `https://www.googleapis.com/auth/classroom.rosters.readonly`
     - `https://www.googleapis.com/auth/classroom.coursework.students.readonly`
     - `https://www.googleapis.com/auth/classroom.student-submissions.students.readonly`
   - Añade usuarios de prueba (para desarrollo)

### 2. Crear credenciales OAuth 2.0

1. En "APIs y Servicios" > "Credenciales"
2. Haz clic en "Crear credenciales" > "ID de cliente de OAuth 2.0"
3. Selecciona "Aplicación de escritorio"
4. Descarga el archivo JSON de credenciales
5. Renómbralo como `client_secret.json`

### 3. Colocar las credenciales

**Para Linux:**
1. Ejecuta los diguientes comandos desde la consola:
    - mkdir -p ~/dirs/classroom-rust/
    - cp client_secret.json ~/dirs/classroom-rust/

**Para Windows:**
1. Copia el archivo client_secret.json a la ruta:
    - C:\Users\{tu_usuario}\AppData\Roaming\classroom-rust\


## 🚀 Instalación y Uso
1. Clona este repositorio. 
    - 
2. Ejecútalo desde la carpeta raíz del repositorio mediante el comando
    - cargo run

La primera vez que ejecutes la aplicación:
1. Se abrirá una ventana del navegador para autenticación
2. Autoriza la aplicación con tu cuenta de Google
3. El token de acceso se guardará automáticamente para futuras ejecuciones

## 📁 Estructura de salida
El programa genera archivos CSV en el directorio scores/ cuyo nombre luce así {nombre_del_curso}.csv

Cada archivo CSV contiene:
- Primera fila: Encabezados (Asignación 1, Asignación 2, ...)
- Primera columna: Nombres de estudiantes
- Celdas: Calificaciones de cada estudiante por asignación

## 🔐 Permisos requeridos
La aplicación requiere los siguientes scopes de Google Classroom:
- classroom.courses.readonly - Ver cursos
- classroom.rosters.readonly - Ver lista de estudiantes
- classroom.coursework.students.readonly - Ver asignaciones
- classroom.student-submissions.students.readonly - Ver entregas y calificaciones

## ⚠️ Notas importantes
- Solo funciona con cuentas de docente
- La primera ejecución requiere interacción manual para la autenticación OAuth
- Los tokens de acceso se refrescan automáticamente
- Asegúrate de que tu proyecto en Google Cloud Console tenga habilitada la API de Classroom
- Los datos se almacenan localmente y no se envían a servidores externos

