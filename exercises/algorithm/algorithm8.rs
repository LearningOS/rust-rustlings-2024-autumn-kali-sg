/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {    // 入队
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {  // 出队
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
    length: usize,
    front: u32,
    back: u32,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            length: 0,
            front: 1,
            back: 2,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.length += 1;
        if self.front == 1{
            self.q1.enqueue(elem);
            while self.q2.size() > 0{
                self.q1.enqueue(self.q2.dequeue().unwrap());
            }
        } else {
            self.q2.enqueue(elem);
            while self.q1.size() > 0{
                self.q2.enqueue(self.q1.dequeue().unwrap());
            }
        }
        let c = self.front;                    // 交换队首队尾标记 
        self.front = self.back;
        self.back = c;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        let queue = if self.back == 2 {&mut self.q2} else {&mut self.q1};
        match queue.dequeue(){         // 队列返回的是Result<T>，用Ok()和Err()处理  // Option用Some()和None处理
            Ok(e) => {
                self.length -= 1;
                Ok(e)
            },
            Err(e) => Err("Stack is empty"),
        }
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.length == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}