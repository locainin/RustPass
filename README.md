# RustPass
Rust-based password generator inspired by the robust and user-friendly KeePassXC interface. This command-line tool is designed for those who require strong, unique passwords without the overhead of a graphical user interface.

# Key Features:

**Customizable Length:** 
*Users can specify the exact length of their desired password, ensuring compatibility with various security requirements.*

**Character Exclusion:** 

Offers the option to exclude specific characters, allowing users to avoid characters that might be problematic in certain contexts.

**Inclusive Character Classes:** *

Supports the inclusion of uppercase letters, lowercase letters, numbers, and special characters, which can be toggled on or off based on user preference.*

**Avoid Lookalikes:** *

Implements a feature to exclude characters that look similar, such as '0' (zero) and 'O' (uppercase 'o'), to prevent confusion.*
    
**Randomization:** 

Leverages the Rust rand crate with OsRng to ensure cryptographic randomness in password generation, providing high-security standards.*

**Simple GUI:** 

*A straightforward command-line interface provides a quick and easy user experience for generating passwords.**


**RustPass** 
*is a lightweight yet powerful tool aiming to fill the gap for a secure, easy-to-use password generator in the Rust ecosystem, taking cues from KeePassXC's reliable approach to password management.*

# **Acknowledgements**

*RustPass is inspired by KeePassXC and aims to bring similar reliability and user-friendliness to the Rust ecosystem. Special thanks to the Rust community for their excellent documentation and support.*
