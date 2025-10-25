extern crate rand;
extern crate csv;
extern crate anyhow;


use rand::prelude::*;
use rand::Rng;
use rand::distr::Uniform;
use std::io::*;

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


fn get_e(grid :  &Grid) -> f64
{
    let mut E = 0f64;
    for y in 0..H-1
    {
        for x in 0..W-1
        {
            if grid.cells[y][x]!=grid.cells[y+1][x] {E+=2.0;}
            if grid.cells[y][x]!=grid.cells[y][x+1] {E+=2.0;}
        }
    }
    E/((W*H*2) as f64)
}


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

    let n_datapoints = 30;
    let T_max = 3.0;
    let T_min = 1.5;
    let n_timestamps = 1000;

    let mut magdata = vec![vec![0.0; n_timestamps]; n_datapoints];
    let mut edata =   vec![vec![0.0; n_timestamps]; n_datapoints];
    let mut tdata = vec![0.0; n_datapoints];

    for t_s in (0..n_datapoints).rev()
    {
        let T = T_min + (T_max-T_min)*((t_s as f64)/(n_datapoints as f64));
        let beta = 1.0/T;

        let rho = 1.0/((beta*2.0).exp()+1.0);

        for _i in 0..20000
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

        tdata[t_s] = T;
        for timestamp in 1..n_timestamps
        {
            for _i in 0..100
            {
                for _j in 0..W
                { if _i%2==_j%2 {bath(_j, &mut grid, rho); }}

                doca(_i%2, &mut grid);
            }

            magdata[t_s][timestamp] = get_mag(&grid);
            edata[t_s][timestamp] = get_e(&grid);
        }
        print!("\rt_s = {t_s}");
        stdout().flush().unwrap();
    }

    println!("done, writing files...");

    let mut magdataw = csv::Writer::from_path("data/magdata.txt").unwrap();
    for i in magdata { magdataw.serialize(i).unwrap(); }

    let mut edataw = csv::Writer::from_path("data/edata.txt").unwrap();
    for i in edata { edataw.serialize(i).unwrap(); }

    let mut tdataw = csv::Writer::from_path("data/tdata.txt").unwrap();
    tdataw.serialize(tdata).unwrap();


}