// Top-level module
module cpu(
    input wire clk,
    input wire rst
);
    // Instantiate the ALU
    alu arithmetic_unit (
        .clk(clk),
        .rst(rst)
    );

    // Instantiate the control unit
    control_unit controller (
        .clk(clk),
        .rst(rst)
    );

    // Instantiate the register file
    register_file registers (
        .clk(clk),
        .rst(rst)
    );
endmodule

// ALU module definition
module alu(
    input wire clk,
    input wire rst
);
    // Instantiate subcomponents
    adder add_unit (
        .clk(clk)
    );

    multiplier mult_unit (
        .clk(clk)
    );
endmodule

// Control unit module
module control_unit(
    input wire clk,
    input wire rst
);
    // Instantiate decoder
    decoder decode (
        .clk(clk)
    );
endmodule

// Register file module
module register_file(
    input wire clk,
    input wire rst
);
endmodule

// Basic components
module adder(
    input wire clk
);
endmodule

module multiplier(
    input wire clk
);
endmodule

module decoder(
    input wire clk
);
endmodule