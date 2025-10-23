//! ```cargo
//! [dependencies]
//! rand = "0.9.0"
//! ```

use rand::prelude::*;
use rand::Rng;
use rand::distr::Uniform;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State
{
    Up,
    Down
}

#[derive(Clone)]
struct Grid
{
    cells : [[State; W]; H],
    b_row : [State; W]
}

const W : usize = 100;
const H : usize = 50;


fn get_mag(grid :  &Grid) -> f64
{
    let mut mup = 0f64;
    for row in grid.cells
    {
        for cell in row
        {
            match cell
            {
                State::Up => mup+=1.0,
                State::Down => mup-=1.0
            }
        }
    }
    mup/((W*H) as f64)
}


//bad accuracy
/*
fn get_e(grid :  &[[State; W]; H]) -> f64
{
    let mut E = 0f64;
    for y in 0..H-1
    {
        for x in 0..W-1
        {
            if grid[y][x]!=grid[y+1][x] {E+=1.0;}
            if grid[y][x]!=grid[y][x+1] {E+=1.0;}
        }
    }
    E/((W*H*2) as f64)
}*/

fn getcelle(x : usize, y : usize, grid :  &Grid) -> i32
{
    let mut out = 0;
    let x = x as i32;
    let y = y as i32;
    for (x1,y1) in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)]
    {
        let v = if 0<=x1 && x1<W as i32 && 0<=y1 && y1<H as i32
        {
            grid.cells[y1 as usize][x1 as usize]
        }
        else if 0<=x1 && x1<W as i32 && y1==H as i32
        {
            grid.b_row[x1 as usize]
        }
        else 
        {
            State::Down
        };
        if v!=grid.cells[y as usize][x as usize] {out+=1;}
    }
    out
}


fn change(x : usize, y : usize, grid :  &mut Grid, beta : f64)
{
    let E0 = getcelle(x,y, grid);
    grid.cells[y][x] = match grid.cells[y][x]
    {
        State::Up => State::Down,
        State::Down => State::Up,
    };
    let E1 = getcelle(x,y, grid);

    if E1<=E0 {return;}
    let p = (-beta*(E1-E0) as f64).exp();

    let mut myrng = rand::rng();
    if myrng.random::<f64>() >p
    {
        grid.cells[y][x] = match grid.cells[y][x]
        {
            State::Up => State::Down,
            State::Down => State::Up,
        };
    }
}

fn bath(x : usize, grid :  &mut Grid, rho : f64)
{
    let c1 = grid.cells[H-1][x];
    let c2 = grid.b_row[x];
    let p = if c1==c2
    {
        rho
    }
    else
    {
        1.0-rho
    };

    let mut myrng = rand::rng();
    if myrng.random::<f64>() < p
    {
        grid.b_row[x] = match grid.b_row[x]
        {
            State::Up => State::Down,
            State::Down => State::Up,
        };
    }
}

fn doca(parity : usize, grid :  &mut Grid)
{
    for y in 0..H
    {
        for x in 0..W
        {
            if parity==(x+y)%2 && getcelle(x,y,grid)==2
            {
                grid.cells[y][x] = match grid.cells[y][x]
                {
                    State::Up => State::Down,
                    State::Down => State::Up,
                };
            }
        }
    }
}


fn main()
{
        println!("hello");

    let mut grid = Grid{cells: [[State::Down; W]; H], b_row: [State::Down; W] };

    let mut myrng = rand::rng();
    let rx = Uniform::<i32>::new(0,W as i32).unwrap();
    let ry = Uniform::<i32>::new(0,H as i32).unwrap();

    for _i in 1..1000
    {
        change(rx.sample(&mut myrng) as usize, ry.sample(&mut myrng) as usize, &mut grid, 0.0);
    }

    for t_s in (100..400).rev()
    {
        let T = t_s as f64/200.0;
        let beta = 1.0/T;

        let rho = 1.0/(beta.exp()+1.0);

        for _i in 1..20000
        {
            for _j in 0..W
            { if _i%2==_j%2 {bath(_j, &mut grid, rho); }}

            doca(_i%2, &mut grid);
        }

       /* for y in 0..H
        {
            for x in 0..W
            {
                print!("{}",match grid.cells[y][x]
                {
                    State::Up => '#',
                    State::Down => ' ',
                });
            }
            println!();
        }
        */

        let m = get_mag(&grid);

        println!("{rho:.3} {m:.3}");
    }
}