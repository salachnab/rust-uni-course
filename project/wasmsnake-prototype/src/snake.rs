use canvas::Canvas;
use direction::Direction;
use stdweb::unstable::TryInto;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

const HEAD_COLOUR: &str = "green";
const TAIL_COLOUR: &str = "lightgreen";
const FOOD_COLOUR: &str = "red";
const FOOD_QUANTITY: u32 = 6;

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Vec<Block>,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x: u32 = js! {return Math.floor(Math.random() * @{width})}.try_into().unwrap();
        let head_y: u32 = js! {return Math.floor(Math.random() * @{height})}.try_into().unwrap();

        let head = Block(head_x, head_y);

        let mut food = vec![];
        for _ in 0..FOOD_QUANTITY {
            let food_x: u32 = js! {return Math.floor(Math.random() * @{width})}.try_into().unwrap();
            let food_y: u32 = js! {return Math.floor(Math.random() * @{height})}.try_into().unwrap();
            food.push(Block(food_x, food_y));
        }

        let tail = Vec::new();

        Snake {
            head,
            tail,
            food,
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.last_direction.opposite(direction) && self.direction.is_none() {
            self.direction = Some(direction)
        } else if self.direction.iter().any(|d| !d.opposite(direction)) {
            self.next_direction = Some(direction)
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        //maybe make the snek cruch into walls?
        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            Direction::Down => Block((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Right => Block((self.head.0 + 1) % self.width, (self.head.1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
        };

        //cut the tail
        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        //check if we bumped into our tail
        if self.tail.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }

        self.head = new_head;

        //check for foods
        for i in 0..FOOD_QUANTITY {
            if self.head == self.food[i as usize] {
                let mut food = self.food[i as usize];
                while food == self.head || self.tail.contains(&food) {
                    let food_x: u32 = js! { return Math.floor(Math.random() * @{self.width}) }.try_into().unwrap();
                    let food_y: u32 = js! { return Math.floor(Math.random() * @{self.height}) }.try_into().unwrap();
    
                    food = Block(food_x, food_y);
                }
                self.food[i as usize] = food;
                //add length
                last_end.map(|x| self.tail.push(x));
            }
        }

        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, HEAD_COLOUR);
        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, TAIL_COLOUR);
        }
        for i in &self.food {
            canvas.draw(i.0, i.1, FOOD_COLOUR);
        }
        
    }
}
