use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    // Mensajes disponibles

    let mut temp = vec![];

    let mut messages = vec![
        "┌───────────────────────────┐",
        "│ Selector de interfaces    │",
        "|---------------------------|",
        "│ Interfaz 1                │",
        "│ Interfaz 2                │",
        "│ Interfaz 3                │",
        "│ Interfaz 4                │",
        "└───────────────────────────┘",
    ];
    let mut interfaz1 = vec![
        "┌───────────────────────────┐",
        "│ primera interfaz          │",
        "└───────────────────────────┘",
    ];

    let mut interfaz2 = vec![
        "┌───────────────────────────┐",
        "│ segunda interfaz          │",
        "└───────────────────────────┘",
    ];

    let mut interfaz3 = vec![
        "┌───────────────────────────┐",
        "│ tercera interfaz          │",
        "└───────────────────────────┘",
    ];

    let mut interfaz4 = vec![
        "┌───────────────────────────┐",
        "│ cuarta interfaz           │",
        "└───────────────────────────┘",
    ];

    // Inicializar la posición del mensaje en "focus"
    let mut selected_index = 3;
    'inicio:loop {
    // Habilitar el modo raw de la terminal
    enable_raw_mode()?;

    // Mostrar los mensajes iniciales
    show_messages(&messages, selected_index)?;

    temp = messages.clone();

    loop{
        // Leer eventos de teclado
        if let Event::Key(KeyEvent { code, modifiers }) = crossterm::event::read()? {
            // Comprobar la tecla presionada
            match code {
                KeyCode::Esc => break 'inicio, // Salir del programa al presionar 'Esc'
                KeyCode::Char('q') => {
                    temp = messages.clone();
                    continue 'inicio; // Volver al inicio del bucle al presionar 'q'
                }
                KeyCode::Up => {
                    // Mover la flecha hacia arriba
                    if selected_index > 3 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Mover la flecha hacia abajo
                    if selected_index < temp.len() - 2 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    // Mostrar la interfaz seleccionada
                    match selected_index {
                        3 => temp = interfaz1.clone(),
                        4 => temp = interfaz2.clone(),
                        5 => temp = interfaz3.clone(),
                        6 => temp = interfaz4.clone(),
                        _ => {}
                    }
                }
                _ => {} // Ignorar otros eventos de teclado
            }

            // Mostrar los mensajes actualizados
            show_messages(&temp, selected_index)?;
        }
    }

    // Deshabilitar el modo raw de la terminal


} Ok(())

}

fn show_messages(temp: &[&str], selected_index: usize) -> Result<()> {
    // Limpiar la pantalla y mover el cursor al inicio
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0))?;

    // Mostrar los mensajes
    for (index, message) in temp.iter().enumerate() {
        if index == selected_index {
            // Agregar una flecha al mensaje seleccionado
            println!("> {}", message);
        } else {
            println!("  {}", message);
        }
    }

    // Mostrar el cursor
    execute!(stdout(), Show)?;

    // Forzar la impresión inmediata en la pantalla
    stdout().flush()?;

    Ok(())
}
