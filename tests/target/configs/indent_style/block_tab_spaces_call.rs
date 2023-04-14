// rustfmt-indent_style: Block
// rustfmt-max_width: 80
// rustfmt-tab_spaces: 2

// #1427
fn main() {
  exceptions::config(move || {
    (
      NmiConfig {},
      HardFaultConfig {},
      SysTickConfig { gpio_sbsrr },
    )
  });
}
