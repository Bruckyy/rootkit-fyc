# Rootkit


---

## Overview

This repository is a proof-of-concept rootkit written in Rust designed to demonstrate common techniques used in rootkits.

The rootkit operates in the kernel space level on Windows systems with a user mode client and implements techniques such as process hiding, token stealing, DSE bypass.

---

## Features

- **Process Hiding** – Hide a specific processes from user-mode monitoring tools such as Task Manager, Process Explorer, and tasklist, by manipulating kernel structures
- **Token Stealing** – Elevates privileges by stealing the token of a process allowing unauthorized escalation to SYSTEM privileges.
- **DSE Bypass** – Bypass DSE by patchig G_CIOptions in kernel memory. This features uses a separate binary that exploit a Write-What-Where vulnerability in the HEVD driver
- **User Mode Client** - A user mode client that issue commands to the rootkit via IRPs.


---

> **Disclaimer**: This project is intended solely for educational and research purposes. Unauthorized use of malicious software is illegal and unethical. This software must not be used outside of controlled, isolated, and authorized environments.

