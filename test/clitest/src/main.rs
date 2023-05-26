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
    let messages = vec![
        "Hola",
        "Cómo estás?",
        "Bienvenido",
        "Rust es genial",
        "Programación es divertida",
    ];

    // Inicializar la posición del mensaje en "focus"
    let mut selected_index = 0;

    // Habilitar el modo raw de la terminal
    enable_raw_mode()?;

    // Mostrar los mensajes iniciales
    show_messages(&messages, selected_index)?;

    loop {
        // Leer eventos de teclado
        if let Event::Key(KeyEvent { code, modifiers }) = crossterm::event::read()? {
            // Comprobar la tecla presionada
            match code {
                KeyCode::Char('q') => break, // Salir del programa al presionar 'q'
                KeyCode::Up => {
                    // Mover la flecha hacia arriba
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Mover la flecha hacia abajo
                    if selected_index < messages.len() - 1 {
                        selected_index += 1;
                    }
                }
                _ => {} // Ignorar otros eventos de teclado
            }

            // Mostrar los mensajes actualizados
            show_messages(&messages, selected_index)?;
        }
    }

    // Deshabilitar el modo raw de la terminal
    disable_raw_mode()?;

    Ok(())
}

fn show_messages(messages: &[&str], selected_index: usize) -> Result<()> {
    // Limpiar la pantalla y mover el cursor al inicio
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    execute!(stdout(), crossterm::cursor::MoveTo(0, 0))?;

    // Mostrar los mensajes
    for (index, message) in messages.iter().enumerate() {
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
