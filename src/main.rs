mod game_engine;
pub use crate::game_engine::game_engine as game;

use macroquad::prelude::*;

const TILESIZE: f32 = 80.0;
const OFFSET:f32 = TILESIZE/2.0;
const WIDTH:f32 = TILESIZE*7.0;
const HEIGHT:f32 = TILESIZE*7.0;
static  COLOURS: [macroquad::color::Color; 23] = [RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, BLACK, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE, GOLD, GRAY, LIGHTGRAY, LIME, MAGENTA, MAROON, PINK, PURPLE, SKYBLUE, WHITE];


#[macroquad::main("ConnectFour")]
async fn main() {
    
    menu().await;

    async fn menu() {
        let mut player_one_colour: usize = 1;
        let mut player_two_colour: usize = 2;

        let menu_items:Vec<String> = vec!["Play".to_string(), "Options".to_string(), "Credits".to_string(), "Quit".to_string()];
        let mut menu_select: usize = 0;
        

        loop {
            request_new_screen_size(WIDTH, HEIGHT);
            clear_background(BLUE);

            if is_key_pressed(KeyCode::Escape) {
                break;
            } 

            if is_key_pressed(KeyCode::Enter) {
                match menu_select {
                    0=>game_loop(player_one_colour,player_two_colour).await,
                    1=>{
                        let tuple = settings(player_one_colour, player_two_colour).await;
                        (player_one_colour, player_two_colour) = tuple;
                    },
                    2=>credits().await,
                    3=>break,
                    _=>(),
                }
            } else if is_key_pressed(KeyCode::Up) && menu_select > 0 {
                menu_select-=1
            } else if is_key_pressed(KeyCode::Down) && menu_select < 3 {
                menu_select += 1
                
            }
            
            draw_text("Connect Four", 20.0, TILESIZE, TILESIZE, BLACK);

            for item in 0..menu_items.len() {
                if item == menu_select {
                    draw_text(&(">".to_owned()+&menu_items[item]), 20.0, (item as f32 + 2.0)*TILESIZE, TILESIZE, DARKGRAY);
                } else {
                    draw_text(&menu_items[item], 20.0, (item as f32 + 2.0)*TILESIZE, TILESIZE, DARKGRAY);

                }
            }
    
            next_frame().await
            
        }
    
        async fn settings(mut p1_colour: usize, mut p2_colour: usize) -> (usize,usize){

            let menu_items:Vec<String> = vec!["Player One Colour".to_string(), "Player Two Colour".to_string(), "Back".to_string()];
            let mut menu_select: usize = 0;
            loop {
                request_new_screen_size(WIDTH, HEIGHT);
                clear_background(BLUE);

                if is_key_pressed(KeyCode::Escape) {
                    return (p1_colour, p2_colour);
                } 

                if menu_select == 2 && is_key_pressed(KeyCode::Enter) {
                    return(p1_colour,p2_colour);
                } else if is_key_pressed(KeyCode::Up) && menu_select > 0 {
                    menu_select-=1
                } else if is_key_pressed(KeyCode::Down) && menu_select < 2 {
                    menu_select += 1       
                } else if is_key_pressed(KeyCode::Left) {
                    match menu_select {
                        0=> {if p1_colour == 0 {
                            p1_colour = 22;
                        } else {p1_colour-=1}},
                        1=> {if p2_colour == 0 {
                            p2_colour = 22;
                        } else {p2_colour-=1}},
                        _=>(),
                    }
                }
                else if is_key_pressed(KeyCode::Right) {
                    match menu_select {
                        0=> {if p1_colour == 22 {
                            p1_colour = 0;
                        } else {p1_colour+=1}},
                        1=> {if p2_colour == 22 {
                            p2_colour = 0;
                        } else {p2_colour+=1}},
                        _=>(),
                    }
                }
                draw_text("Options", 20.0, TILESIZE, TILESIZE, BLACK);

                for item in 0..menu_items.len() {
                    if item == menu_select {
                        draw_text(&(">".to_owned()+&menu_items[item]), 20.0, (item as f32 + 4.0)*TILESIZE/1.5-50.0, TILESIZE/2.0, DARKGRAY);
                    } else {
                        draw_text(&menu_items[item], 20.0, (item as f32 + 4.0)*TILESIZE/1.5-50.0, TILESIZE/2.0, DARKGRAY);

                    }
                }
                draw_circle(375.0,(4.0)*TILESIZE/1.5-55.0, 20.0, COLOURS[p1_colour]);
                draw_circle(375.0,(5.0)*TILESIZE/1.5-55.0, 20.0, COLOURS[p2_colour]);
                draw_text("<   >", 330.0,(4.0)*TILESIZE/1.5-50.0,  TILESIZE/2.0, BLACK);
                draw_text("<   >", 330.0,(5.0)*TILESIZE/1.5-50.0,  TILESIZE/2.0, BLACK);
        
                next_frame().await
                
            }
        }
   
        async fn credits() {

            let menu_items:Vec<String> = vec!["A game made by Cameron Garvey".to_string(), "https://github.com/camerongarvey".to_string(), 
                                            "Graphics made using the Macroquad library".to_string(), 
                                            "https://crates.io/crates/macroquad".to_string(), "All Trademarks to their respective owner".to_string(), 
                                            "This product is not for commercial use".to_string(),"Back".to_string()];
            let mut press_delay = false;
            
            loop {
                request_new_screen_size(WIDTH, HEIGHT);
                clear_background(BLUE);

                if is_key_pressed(KeyCode::Escape) {
                    break;
                } 
                if press_delay && is_key_pressed(KeyCode::Enter) {
                    break
                } if !press_delay && is_key_released(KeyCode::Enter) {
                    press_delay = true;
                }
                draw_text("Credits", 20.0, TILESIZE, TILESIZE, BLACK);

                for item in 0..menu_items.len() {
                    if item == menu_items.len()-1 {
                        draw_text(&(">".to_owned()+&menu_items[item]), 20.0, ((item as f32 + 2.0)*TILESIZE/3.0)+60.0, TILESIZE/2.5, DARKGRAY);
                    } else {
                        draw_text(&menu_items[item], 20.0, ((item as f32 + 2.0)*TILESIZE/3.0) +60.0, TILESIZE/3.0, BLACK);
                        }
                    }
        
                next_frame().await
                }
            }
        
        async fn game_loop(p1_colour: usize, p2_colour: usize) {
                let mut board = game::Board::new();
                let mut current_colunm: f32 = 1.0;
                let mut turn_colour:macroquad::color::Color = COLOURS[p1_colour];
                let mut status:u8 = 0;  
                let mut press_delay = false;
                loop {
                    if is_key_pressed(KeyCode::Escape) {
                        break;
                    } if !press_delay && is_key_released(KeyCode::Enter) {press_delay=true}
                   
                    
                    clear_background(BLUE);
        
                    for tile_x in  0..7 {
                        for tile_y in 0..6 {
                            let tile_colour: macroquad::color::Color;
                            if board.board[tile_x][tile_y] == 'r'  {tile_colour=COLOURS[p1_colour]}
                            else if board.board[tile_x][tile_y] == 'y' {tile_colour=COLOURS[p2_colour]}
                            else {tile_colour=WHITE}
                            draw_circle(tile_x as f32*TILESIZE+OFFSET, tile_y as f32*TILESIZE + OFFSET*3.0, TILESIZE/2.0, tile_colour);
                        }
        
                    }
        
                    if status == 0 {
                        draw_circle(current_colunm*TILESIZE-OFFSET, TILESIZE/2.0, TILESIZE/2.0, turn_colour);
                        if is_key_pressed(KeyCode::Right) && current_colunm<7.0 {
                            current_colunm+=1.0;
                        } else if is_key_pressed(KeyCode::Left) && current_colunm>1.0{
                            current_colunm-=1.0;
                        }
                        if press_delay && is_key_pressed(KeyCode::Enter) {
                            if board.check_legal_move(current_colunm as usize) {
                                board.make_move(current_colunm as usize);
                                if board.turn == 'r' {
                                    board.turn ='y';
                                    turn_colour = COLOURS[p2_colour]
                                } else {
                                    board.turn='r';
                                    turn_colour=COLOURS[p1_colour];
                                }
            
                                if board.check_win(current_colunm as usize) {
                                    if board.turn == 'y' {
                                        status = 1;
                                        press_delay = false;
                        
                                    } else {
                                        status = 2; 
                                        press_delay = false;                      
                                    }
                                } else if !board.board.iter().any(|row| row.contains(&'0')){
                                    status = 3;
                                    press_delay = false;
                                }
                            }
                        }
                    } else {
                        
                        match status {
                            1=>draw_text("Player 1 Wins!", 20.0, 20.0, 30.0, DARKGRAY),
                            2=>draw_text("Player 2 Wins!", 20.0, 20.0, 30.0, DARKGRAY),
                            3=>draw_text("It's a tie!", 20.0, 20.0, 30.0, DARKGRAY),
                            _=>()              
                        }
                        draw_text("Press Enter to go to the Menu", 20.0, 40.0, 30.0, DARKGRAY);
        
                        if !press_delay && is_key_released(KeyCode::Enter) {press_delay=true}
                        if press_delay && is_key_down(KeyCode::Enter) {break}
                    }
        
                    next_frame().await
                }
            }
    
    }

    
    //async

} 
