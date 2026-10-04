use std::cell::Cell;
use std::rc::Rc;

use ConcreteBlock;

struct DropProbe(Rc<Cell<usize>>);

impl DropProbe {
    fn drops(&self) -> usize {
        self.0.get()
    }
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn stack_block_calls_with_captures_and_drops_them_once() {
    let drops = Rc::new(Cell::new(0));
    let probe = DropProbe(Rc::clone(&drops));
    let block = ConcreteBlock::new(move |value: usize| value + probe.drops());

    assert_eq!(unsafe { block.call((17,)) }, 17);
    assert_eq!(drops.get(), 0);
    drop(block);
    assert_eq!(drops.get(), 1);
}

#[test]
fn copied_block_owns_capture_until_last_runtime_reference_is_released() {
    let drops = Rc::new(Cell::new(0));
    let block = {
        let probe = DropProbe(Rc::clone(&drops));
        ConcreteBlock::new(move |value: usize| value + probe.drops()).copy()
    };
    let retained = block.clone();

    assert_eq!(unsafe { block.call((23,)) }, 23);
    assert_eq!(drops.get(), 0);
    drop(block);
    assert_eq!(drops.get(), 0);
    assert_eq!(unsafe { retained.call((41,)) }, 41);
    drop(retained);
    assert_eq!(drops.get(), 1);
}
