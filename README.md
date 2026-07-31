# Useful Things

## Programming Infrastructure

- Python Programming
  - [vulture][vulture]: Find dead python code
  - [pylama][pylama]: Useful python code-audit tool
  - [ty][ty]: A Python type checker in rust
  - [uv][uv]: A rust replacement for `pip`
  - [pyupgrade][pyupgrade]: Automatically upgrade python syntax to baseline against specific python versions
  - [bandit][bandit]: Find security problems in python code
  - [complexipy][complexipy]: Rapidly find Python methods that are too complex
  - [ruff][ruff]: Lint python quickly
  - [pss][pss]: Code-search tool written in python
  - [python-inject][inject-python]: Inject python dependencies into pytests
- Python Debuggers / Profilers
  - [pudb][pudb]: Full-screen terminal Python debugger
  - [pdoc][pdoc]: Documentation system for moderate-sized python projects
  - [PySnooper][pysnooper]: Debug python with decorators
  - [snoop][snoop]: Debug python with decorators; similar to PySnooper
  - [pyinstrument][pyinstrument]: Call stack profiler for python libraries; detect why python is slow
  - [py-spy][py-spy]: Sampling profiler for python code
- Python Compilers / Transpilers
  - [codon][codon]: Compile Python into static binaries
  - [PyO3][pyo3]: Write Python modules in rust, or embed Python in a rust binary
  - [Cython][cython]: Accelerate Python with C
  - [py2many][py2many]: A python transpiler (i.e. convert Python to Rust, Go, C++, etc...)
  - [py2exe][py2exe]: Compile Python to Windows binaries
  - [pyinstaller][pyinstaller]: Compile to binary including required packages
  - [Nuitka][nuitka]: Compile python code to a binary
  - [edgepython][edgepython]: Compile a subset of python to a WASM binary to run "python" in browser / linux. Caution, no STDLIB included.
- YAML
  - [yamllint][yamllint]: Simple yaml linter
  - [yamlfmt][yamlfmt]: Google's yaml linter / formatter
  - [yamlfix][yamlfix]: Automatic yaml formatting fixer
- Markdown
  - [rumdl][rumdl]: Markdown Linter
- VIM / IDE
  - [ALE][ale]: Add IDE features to `vim` and `neovim`
  - [Neovide][neovide]: Enhanced neovim GUI with a focus on vim as an IDE
  - [nvim-treesitter][nvim-treesitter]: Parser and auto-complettion for neovim
  - [YouCompleteMe][youcompleteme]: Auto-completion for ALE
- Rust
  - [rust-analyzer][rust-analyzer]: Linter for rust projects
- [Jujitsu][jujitsu]: A trendy alternative to git for DCVS; developed at Google.

## Languages / Toolkits

- [OpenTofu][opentofu]: Open-source alternative to Terraform
- [Ansible][ansible]
- [Deno][deno]: Security-focused Rust JavaScript run-time; memory-safe alternative to Node.js

### GitHub Actions

- [act][act]: Run GitHub actions locally on your laptop
- [zizmor][zizmor]: Static YAML analysis for GitHub Actions YAML

### Cloud Infrastructure

- [checkov][checkov]: Lint your Terraform, Dockerfiles and more
- [ScoutSuite][ScoutSuite]: Multi-cloud security auditing
- [Terminal AWS][taws]: Manage your AWS infrastructure in the terminal

### AI Toolkit

- [rtk][rtk]: CLI proxy that reduces LLM token consumption by 60-90% on common dev commands.

## Python Language

### Python: Code Infrastructure

- Libraries
  - [click][click]: Flexible CLI arguments library as an improvement over `argparse`
  - [attrs][attrs]: Python classes without boilerplate
  - [traitlets][traitlets]: Python classes with strong typing and attribute observer operations.
  - [loguru][loguru]: Delgan's excellent Python logging framework
  - [transitions][transitions]: Object-oriented pythonic State Machine
  - [dotenv][dotenv]: Manage / read `.env` files in Python
- Data Validation / Serialization
  - [pydantic][pydantic]: Python data validation inside classes
  - [marshmallow][marshmallow]: Implement Python object serialization an de-serialization with type validation; this library is oriented around Python objects as dicts.
  - [adaptix][adaptix]: Simplified Python data validation and conversion between sqlite, dict, dataclasses
- Python Web Development
  - [flask][flask]: Simple python web-development framework
  - [waitress][waitress]: A production-grade WSGI web server (useful to serve flask apps in production)
  - [dominate][dominate]: Write HTML with python
  - [scrapy][scrapy]: Web scraping framework
- [typeguard][typeguard]: Python runtime type checker; needs a maintainer after v4.0.0
- [gRPC][grpc]: Fast, typed Python message passing library
- [msgspec][msgspec]: Fast Python object serialization / deserialization with type checking; partially implemented in C. Data is supported in json, yaml, and [MessagePack][msgpack-py], and toml.
- [msgpack][msgpack-py]: MessagePack for Python is an efficient binary serialization format. It lets you exchange data among multiple formats. See [msgspec][msgspec] for more detail.
- [trio][trio]: Python async and concurrency library

### Python: General Usage

- [pre-commit][pre-commit]: Add hooks to git commit actions
- [ultisnips][ultisnips]
- [cookiecutter][cookiecutter]: A disk and file templating framework
- [pyautogui][pyautogui]: Framework to exert control over GUI interfaces
- [pywinauto][pywinauto]: Framework to exert control over GUI interfaces
- [jupyter-notebook][jupyter_notebook]: Dynamically analyze python code as a savable notebook
- [GitPython][gitpython]: Manage git repositories with python
- [rich][rich]: Library for terminal colorization
- [ptftpd][ptftpd]: Python TFTPd and PXE tool suite
- [pyjwt][pyjwt]: Python implementation of JWT (JSON Web Token)
- [python-saml][python-saml]: Python SAML toolkit
- [dnspython][dnspython]: Python interface to DNS queries
- [splunk-sdk-python][splunk-sdk-python]: Splunk SDK for Python
- [hvac][hvac]: Python interface to Hashicorp Vault
- [pyshark][pyshark]: Python interface to `tshark` / Wireshark in the Terminal
- [docker-py][docker-py]: Python API for the Docker ecosystem
- [psutil][psutil]: Cross platform process and system monitoring
- [watchdog][watchdog]: Monitor file-system events in python
- [invoke][invoke]: Pythonic command execution and task management
- [pysmb][pysmb]: Python SMB client
- [pexpect][pexpect]: Pythonic Expect library
- [pyparsing][pyparsing]: Popular python parsing library via PEG parser
- [TextFSM][textfsm]: Stateful templated python text parsing and value extraction
- [ntc-templates][ntc-templates]: Templates to parse command output via [TextFSM][textfsm]
- [fuzzywuzzy][fuzzywuzzy]: Fuzzy text matching
- [rich][rich]: Command-line text formatting library
- [python-prompt-toolkit][python_prompt_toolkit]: Build interactive Python command-line programs
- [deepdiff][deepdiff]: Diff python nested dicts
- [dictdiffer][dictdiffer]: Diff python dicts
- [PyGithub][PyGithub]: GitHub v3 API python library
- [atlassian-python-api][atlassian-python-api]: REST API for Atlassian products (i.e. Jira, Confluence)
- [landslide][landslide]: Markdown and Python-based slide deck library
- Database / SQL / ORM
  - [peewee][peewee]: Python ORM for sqllite, postgresql, mysql and cockroach DB
  - [SQLAlchemy][sqlalchemy]: The Database Toolkit for Python
  - [records][records]: Make SQL queries and ingest results as python objects
- JSON / YAML
  - [orjson][orjson]: Fast and correct json parsing library
  - [pyyaml][pyyaml]: Pythonic yaml handling
  - [ruamel.yaml][ruamel-yaml]: Pythonic yaml handling
- Time Utilities (mostly useless after python stdlib introduced `zoneinfo`)
  - [whenever][whenever]: Time-handling replacement for Python datetime and some [Arrow][arrow] use-cases
  - [Arrow][arrow]: Flexible Python time-handling
- Cron / Durable Execution
  - [absurd][absurd]: Durable execution engine for reliable workflows / CICD; written by Armin Ronacher.
  - [croniter][croniter]: Parses cron schedules to iterate over datetime objects.
  - [APScheduler][apscheduler]: Python Task scheduling library
  - [rocketry][rocketry]: Pythonic scheduling and cron-like recur using python decorators
- Graphing / Diagrams
  - [plotly][plotly]: Graphing and plotting library
  - [matplotlib][matplotlib]: Build graphs with Python
  - [Seaborn][seaborn]: Simple statistical graphing package
  - [diagrams][diagrams]: Diagrams as Python code
  - [mermaid-py][mermaid-py]: Python interface to `mermaid-js` diagramming library
- Data Science / Computation
  - [pandas][pandas]: Pythonic data manipulation and statistical analysis library
  - [polars][polars]: Python data manipulation and statistical analysis written in rust
  - [prophet][prophet]: Facebook's canned time-series forcasting library

### Python: Network Automation

- [fabric][fabric]:  Automate command execution on linux systems via SSH
- [boto3][boto3]:  Automate AWS environments
- [netmiko][netmiko]:  Automate command execution on routers / switches via SSH
- [nornir][nornir]:  Network inventory and execution abstractions
- [exabgp][exabgp]:  BGPd written in Python
- [scrapli][scrapli]:  Automate command execution on routers / switches via SSH
- [suzieq][suzieq]:  Canned network metrics collection
- [stockpiler][stockpiler]: Brett Lykins' config backup tool.  Only supports Cisco and F5.
- [meraki-cli][meraki-cli]:  Meraki CLI tool for interacting with the dashboard
- [py-junos-eznc][py-junos-eznc]:  Junos automation library

### Machine Learning

- [keras][keras]
- [scikit-learn][scikit-learn]
- [langchain][langchain]: Framework for building AI Agents
- [textblob][textblob]: Text processing and sentiment analysis
- [MLAlgorithms][MLAlgorithms]: Clean Machine Learning examples

## Rust Language

- Tutorials
  - [Comprehensive Rust][comprehensive_rust]: Google's Rust Tutorial
  - [Rust for the Impatient][rustfortheimpatient]: 10-minute YouTube video
  - [Rust in half an hour][rustin30minutes]
  - The best single-source of rust examples is [Rust By Example][rust-by-example].

### General Rust libraries

- [rexpect][rexpect]: Rust expect library
- [textfsm-rust][textfsm-rust]: Rust port of Google's TextFSM Python library
- [rust-loguru][rustloguru]: Rust logging library
- [chrono][chrono]: Rust date and time handling
- [rustyline][rustyline]: Rust replacement for linux `readline`
- [pest parser][pestparser]: A popular Rust parser
- [parking_lot][parking_lot]: Efficient Rust synchronozation primatives - `Mutex`, `RwLock`, etc... these are faster than stdlib equivalents
- [colored][colored]: Text color support
- [hickory-dns][hickory-dns]: A Rust DNS client, server and resolver
- [rustix][rustix]: Replace OS system calls with memory-safe alternatives; no libc binding required.
- GUI Frameworks
  - [slint][slint]: Build cross-platform user interfaces with clients for Rust, C++, JavaScript and Python; some applications require a paid license.
- CLI Arguments
  - [clap][clap]: Full-featured Rust CLI arg parsing
  - [argh][argh]: Simplified Rust CLI arg parsing
- Error Handling
  - [snafu][snafu]: Rust error handling
  - [anyhow][anyhow]: Rust Application error handling
  - [thiserror][thiserror]: Rust Application error handling

## JavaScript / CSS / Web

- [ECharts][echarts]: Vue-based charting library
- [Vue][vuejs]: Vue.js is a progressive, incrementally-adoptable JavaScript framework for building UI on the web.

## Go Language

- [go-charts][go-charts]: Go charts library
- [plot][plot]: A repository for plotting and visualizing data
- [delve][delve]: Go debugger; also see the [delve youtube demo](https://www.youtube.com/watch?v=a1SneuI65O0)
- [goph][goph]: A Go SSH client

## Info

- [what-happens-when][what-happens-when]: A detailed description of what happens when you type "google.com" into your browser and press Enter.

## Tools

### General Use utilities

- [zellij][zellij]: A rust terminal multiplexer
- [evil-helix][evilhelix]: The Helix editor with vim bindings.
- [atuin][atuin]: Sync all shell history across sessions and optionally store encrypted in the cloud
- [difftastic][difftastic]: A text file differ that understands many programming languages. Diffs are intelligent based on the language in the file.
- [taskwarrior][taskwarrior]: A task-list manager with a useful CLI
- [just][just]: `just` is like `make`, but without the `Makefile` syntax warts
- [ripgrep][ripgrep]: Recursive grep through directories
- [sd][sd]: A simpler sed-like command, implemented in Rust
- [RustScan][RustScan]: A fast port-scanner in Rust (can replace `nmap` for some tasks)
- [reveal.js][reveal.js]: HTML and JS-based slide deck
- [starship][starship]: Dynamic rust-powered shell prompts with meaningful info
- [Win11Debloat][win11debloat]: Powershell script to disable Microsoft trackers and other Win11 bloatware.
- [warp][warp]: Windows / Linux / MacOS Shell with embedded AI agent capabilities.
- [wasmtime][wasmtime]: Run wasm binaries on linux
- [gobackup][gobackup]: Server backup automation that uses anything from local storage to Amazon S3
- [podman][podman]: Run Docker / OCI containers without root; replaces Docker for many workflows.
- [rustpbx][rustpbx]: Rust PBX / Session Border Controller
- [stress-ng][stress-ng]: Load-test CPU, Memory, Threading, Disk I/O
- [rustdesk][rustdesk]: An open-source remote desktop application designed for self-hosting, as an alternative to TeamViewer.
- [lettre][lettre]: A rust SMTP mailer
- [sniffglue][sniffglue]: A rust packet sniffer; utilizes all CPU cores. The user interface isn't great; does not use standard pcap filltering args.
- System Monitoring
  - [bottom][bottom]: Flexible, Rust-based terminal monitoring tool. Favorite invocation - `btm -bT`
  - [htop][htop]: Multi-core evolution of top
  - [procs][procs]: List processes, rust replacement for `ps`
  - [rustnet-monitor][rustnet]: Per-process network monitoring for your terminal: live TCP, UDP, and QUIC connections with deep packet inspection, sandboxed by default.
  - [trippy][trippy]: Traceroute / [mtr][mtr] written in Rust

### Terminal utilites

- [asciienema][asciienema]: A terminal session recorder built with Rust
- [obs-studio][obs-studio]: Record YouTube videos from your laptop

### General Tools: Services

- [VictoriaMetrics][victoriametrics]: Drop-in replacement for Prometheus. Highly scalable for large data sets
- [perses][perses]: Forget Grafana, manage graph dashboards in git as code
- [openobserve][openobserve]: Efficient log collection and management with up to 170x log storage compression ratios
- [smallstep][smallstep]: CLI TLS Certificate Authority (CA)
- [scylladb][scylladb]: Fast NoSQL DB

### Network Tools: Services

- [caddy][caddy]: Go webserver / reverse proxy with auto-TLS certificate generation
- [frp][frp]: A Golang fast reverse proxy to help you expose a local server behind a NAT or firewall to the internet.
- [nsd][nsd]: DNS server in C
- [coredns][coredns]: Golang DNS server that supports UDP, DoT, DoH, DoQUIC and more.
- [unbound][unbound]: Popular DNS resolver library
- [httptap][httptap]: Get a terminal waterfall diagram of website responses
- [Netbox][netbox]: Network source of truth; based on Django
- [NIPAP][nipap]: Open-source IP address management (IPAM)
- [fail2ban][fail2ban]: Monitor and black-hole repetitive attacks on your Linux server
- [maltrail][maltrail]: Detect malicious network traffic
- [secure_cartography][secure-cartography]: Use CDP / LLDP / SSH to compile accurate network diagrams
- [influxdb][influxdb]: A time-series database
- [FRrouting][frr]: The FRRouting Protocol Suite; open implementations of BGP, OSPF, EIGRP, PIM, etc... Use `vtysh` to manage the various routing daemons.

### Network Tools: Diagnostics and Testing

- Ping / Traceroute
  - [trippy][trippy]: Traceroute / [mtr][mtr] written in Rust
  - [mtr][mtr]: Traceroute / mtr written in Rust
  - [latency-monitor][latency-monitor]: One or two-way latency measurements
  - [gping][gping]: Graph ping response times (written in Rust) in the terminal window
  - [pingtracer][pingtracer]: Progressive ping logger written for MS Windows; discovers all hops in the path and pings them
- Proxies
  - [mitmproxy][mitmproxy]: An interactive TLS-capable intercepting HTTP proxy for penetration testers and software developers.
  - [go-mitmproxy][go-mitmproxy]: A port of [mitmproxy][mitmproxy] to Go
- Sniffers
  - [sniffnet][sniffnet]: Cross-platform network traffic capture with a nice GUI (Windows / MacOS / Linux)
- Monitoring / NMS
  - [librenms][librenms]: One stop shop for monitoring Cisco / Arista / etc...
  - [Rust-Ping][rust-ping]: Open source NMS written in Rust
  - [network-monitor][network-monitor]: Open source NMS written in Rust
  - [nethogs][nethogs]: Detect which linux processes are consuming the most bandwidth
  - [bandwhich][bandwhich]: Detect which linux processes are consuming the most bandwidth (written in Rust)
  - [sniffer][sniffer]: Detect which linux / Windows processes are consuming the most bandwidth (written in Go)
  - [rrdtool][rrdtool]: A time-series database, which intelligently compresses data
- Network Emulation
  - [containerlab][containerlab]: Build arbitrary topologies of your favorite Cisco / Arista / Juniper / Nokia Operating System as a Docker container
  - [vrnetlab][vrnetlab]: Tool to convert router images to a Docker container
- Network Testing
  - [flent][flent]: A network performance test client, which requires [fping][fping] (client-side), [netperf][netperf] (server-side) or [iperf2][iperf2] (server-side)
  - [iperf2][iperf2]: Classic network test tool
  - [iperf3][iperf3]: Next-gen of iperf
  - [crusader][crusader]: Rust-based network testing and results graphing
  - [goben][goben]: Measure TCP / UDP throughput between hosts
  - [TRex][trex]: Cisco's flexible Network test tool
  - [netperf][netperf]: A network performance test server, written by Hewlett Packard. It's best to install the Debian package instead of trying to compile [netperf][netperf]

### Embedded Operating Systems

- [Zephyr][zephyros]: A popular RTOS for Embedded systems
- [Embassy][embassy]: A rust embedded operating system
- [rtic][rtic]: RTOS for ARM Cortex-M microcontrollers
- [Tock][tock]: A secure RTOS for embedded system

  [pysnooper]: https://github.com/cool-RR/PySnooper
  [traitlets]: https://github.com/ipython/traitlets
  [ultisnips]: https://github.com/SirVer/ultisnips
  [keras]: https://github.com/keras-team/keras
  [langchain]: https://github.com/langchain-ai/langchain
  [scrapy]: https://github.com/scrapy/scrapy
  [py2many]: https://github.com/py2many/py2many
  [diagrams]: https://github.com/mingrammer/diagrams
  [cookiecutter]: https://github.com/cookiecutter/cookiecutter
  [prophet]: https://github.com/facebook/prophet
  [fabric]: https://github.com/fabric/fabric
  [netmiko]: https://github.com/ktbyers/netmiko
  [invoke]: https://github.com/pyinvoke/invoke
  [loguru]: https://github.com/Delgan/loguru
  [scrapli]: https://github.com/carlmontanari/scrapli
  [vulture]: https://github.com/jendrikseipp/vulture
  [gitpython]: https://github.com/gitpython-developers/GitPython
  [marshmallow]: https://github.com/marshmallow-code/marshmallow/
  [pyupgrade]: https://github.com/asottile/pyupgrade
  [pylama]: https://github.com/klen/pylama
  [atlassian-python-api]: https://github.com/atlassian-api/atlassian-python-api
  [pyautogui]: https://github.com/asweigart/pyautogui
  [whenever]: https://github.com/ariebovenberg/whenever
  [arrow]: https://github.com/arrow-py/arrow
  [pandas]: https://github.com/pandas-dev/pandas
  [jupyter_notebook]: https://github.com/jupyter/notebook
  [pyyaml]: https://github.com/yaml/pyyaml
  [seaborn]: https://github.com/mwaskom/seaborn
  [textfsm]: https://github.com/google/textfsm
  [ale]: https://github.com/dense-analysis/ale
  [boto3]: https://github.com/boto/boto3
  [click]: https://github.com/pallets/click
  [rich]: https://github.com/Textualize/rich
  [python_prompt_toolkit]: https://github.com/prompt-toolkit/python-prompt-toolkit
  [deepdiff]: https://github.com/seperman/deepdiff
  [dotenv]: https://github.com/theskumar/python-dotenv
  [transitions]: https://github.com/pytransitions/transitions
  [dominate]: https://github.com/Knio/dominate
  [cython]: https://github.com/cython/cython
  [pyo3]: https://github.com/PyO3/pyo3
  [polars]: https://github.com/pola-rs/polars
  [matplotlib]: https://github.com/matplotlib/matplotlib
  [py-spy]: https://github.com/benfred/py-spy
  [pyinstaller]: https://github.com/pyinstaller/pyinstaller
  [zizmor]: https://github.com/zizmorcore/zizmor
  [act]: https://github.com/nektos/act
  [neovide]: https://github.com/neovide/neovide
  [scikit-learn]: https://github.com/scikit-learn/scikit-learn
  [plotly]: https://github.com/plotly/plotly.py
  [pre-commit]: https://github.com/pre-commit/pre-commit
  [rust-analyzer]: https://github.com/rust-lang/rust-analyzer
  [nuitka]: https://github.com/Nuitka/Nuitka
  [peewee]: https://github.com/coleifer/peewee
  [psutil]: https://github.com/giampaolo/psutil
  [MLAlgorithms]: https://github.com/rushter/MLAlgorithms
  [python-prompt-toolkit]: https://github.com/prompt-toolkit/python-prompt-toolkit
  [Text-Blob]: https://github.com/sloria/TextBlob
  [fuzzywuzzy]: https://github.com/seatgeek/fuzzywuzzy
  [checkov]: https://github.com/bridgecrewio/checkov
  [orjson]: https://github.com/ijl/orjson
  [PyGithub]: https://github.com/PyGithub/PyGithub
  [bandit]: https://github.com/PyCQA/bandit
  [ScoutSuite]: https://github.com/nccgroup/ScoutSuite
  [records]: https://github.com/kennethreitz/records
  [docker-py]: https://github.com/docker/docker-py
  [watchdog]: https://github.com/gorakhargosh/watchdog
  [trio]: https://github.com/python-trio/trio
  [pywinauto]: https://github.com/pywinauto/pywinauto
  [attrs]: https://github.com/python-attrs/attrs
  [pyjwt]: https://github.com/jpadilla/pyjwt
  [yamllint]: https://github.com/adrienverge/yamllint
  [yamlfix]: https://github.com/lyz-code/yamlfix
  [pudb]: https://github.com/inducer/pudb
  [pexpect]: https://github.com/pexpect/pexpect
  [dnspython]: https://github.com/rthalley/dnspython
  [pyshark]: https://github.com/KimiNewt/pyshark
  [pyparsing]: https://github.com/pyparsing/pyparsing
  [pdoc]: https://github.com/mitmproxy/pdoc
  [rocketry]: https://github.com/Miksus/rocketry
  [textblob]: https://github.com/sloria/textblob
  [codon]: https://github.com/exaloop/codon
  [pyinstrument]: https://github.com/joerick/pyinstrument
  [exabgp]: https://github.com/Exa-Networks/exabgp
  [nornir]: https://github.com/nornir-automation/nornir
  [snoop]: https://github.com/alexmojaki/snoop
  [hvac]: https://github.com/hvac/hvac
  [ntc-templates]: https://github.com/networktocode/ntc-templates
  [suzieq]: https://github.com/netenglabs/suzieq
  [dictdiffer]: https://github.com/inveniosoftware/dictdiffer
  [splunk-sdk-python]: https://github.com/splunk/splunk-sdk-python
  [py-junos-eznc]: https://github.com/Juniper/py-junos-eznc
  [python-saml]: https://github.com/SAML-Toolkits/python-saml
  [pysmb]: https://github.com/miketeo/pysmb
  [pss]: https://github.com/eliben/pss
  [staticdhcpd]: https://github.com/flan/staticdhcpd
  [ptftpd]: https://github.com/mpetazzoni/ptftpd
  [meraki-cli]: https://github.com/PackeTsar/meraki-cli
  [mermaid-py]: https://github.com/ouhammmourachid/mermaid-py
  [caddy]: https://github.com/caddyserver/caddy
  [trippy]: https://github.com/fujiapple852/trippy
  [mtr]: https://github.com/traviscross/mtr
  [pingtracer]: https://github.com/bp2008/pingtracer
  [gping]: https://github.com/orf/gping
  [nethogs]: https://github.com/raboof/nethogs
  [bandwhich]: https://github.com/imsnif/bandwhich
  [sniffer]: https://github.com/chenjiandongx/sniffer
  [mitmproxy]: https://github.com/mitmproxy/mitmproxy
  [go-mitmproxy]: https://github.com/lqqyt2423/go-mitmproxy
  [flent]: https://github.com/tohojo/flent
  [netperf]: https://github.com/HewlettPackard/netperf
  [iperf3]: https://github.com/esnet/iperf
  [iperf2]: https://sourceforge.net/projects/iperf2/
  [sniffnet]: https://github.com/GyulyVGC/sniffnet
  [asciienema]: https://github.com/asciinema/asciinema
  [obs-studio]: https://github.com/obsproject/obs-studio
  [difftastic]: https://github.com/Wilfred/difftastic
  [taskwarrior]: https://github.com/GothenburgBitFactory/taskwarrior
  [influxdb]: https://github.com/influxdata/influxdb/
  [go-charts]: https://github.com/go-echarts/go-echarts
  [rrdtool]: https://github.com/oetiker/rrdtool-1.x
  [just]: https://github.com/casey/just
  [netbox]: https://github.com/netbox-community/netbox
  [nipap]: https://github.com/SpriteLink/NIPAP
  [sd]: https://github.com/chmln/sd
  [ripgrep]: https://github.com/BurntSushi/ripgrep
  [fail2ban]: https://github.com/fail2ban/fail2ban
  [maltrail]: https://github.com/stamparm/maltrail
  [RustScan]: https://github.com/bee-san/RustScan
  [snafu]: https://github.com/shepmaster/snafu
  [rexpect]: https://github.com/rust-cli/rexpect
  [nvim-treesitter]: https://github.com/nvim-treesitter/nvim-treesitter
  [youcompleteme]: https://github.com/ycm-core/YouCompleteMe
  [anyhow]: https://github.com/dtolnay/anyhow
  [thiserror]: https://github.com/dtolnay/thiserror
  [argh]: https://github.com/google/argh
  [clap]: https://github.com/clap-rs/clap
  [rust-by-example]: https://doc.rust-lang.org/rust-by-example/
  [ruamel-yaml]: https://pypi.org/project/ruamel.yaml/
  [plots]: https://github.com/gonum/plot
  [crusader]: https://github.com/Zoxc/crusader
  [goben]: https://github.com/udhos/goben
  [trex]: https://github.com/cisco-system-traffic-generator/trex-core
  [nsd]: https://github.com/NLnetLabs/nsd
  [coredns]: https://github.com/coredns/coredns
  [unbound]: https://github.com/NLnetLabs/unbound
  [frp]: https://github.com/fatedier/frp
  [containerlab]: https://github.com/srl-labs/containerlab
  [vrnetlab]: https://github.com/vrnetlab/vrnetlab
  [secure-cartography]: https://github.com/scottpeterman/secure_cartography
  [scylladb]: https://github.com/scylladb/scylladb
  [embassy]: https://github.com/embassy-rs/embassy
  [zephyros]: https://github.com/zephyrproject-rtos/zephyr
  [rtic]: https://github.com/rtic-rs/rtic
  [tock]: https://github.com/tock/tock
  [uv]: https://github.com/astral-sh/uv
  [ty]: https://github.com/astral-sh/ty
  [bottom]: https://github.com/ClementTsang/bottom
  [zellij]: https://github.com/zellij-org/zellij
  [reveal.js]: https://github.com/hakimel/reveal.js
  [landslide]: https://github.com/adamzap/landslide
  [starship]: https://github.com/starship/starship
  [win11debloat]: https://github.com/Raphire/Win11Debloat
  [warp]: https://github.com/warpdotdev/Warp
  [delve]: https://github.com/go-delve/delve
  [typeguard]: https://github.com/agronholm/typeguard
  [py2exe]: https://github.com/py2exe/py2exe
  [pydantic]: https://github.com/pydantic/pydantic
  [msgspec]: https://github.com/jcrist/msgspec
  [msgpack-py]: https://github.com/msgpack/msgpack-python
  [adaptix]: https://github.com/reagento/adaptix
  [what-happens-when]: https://github.com/alex/what-happens-when
  [latency-monitor]: https://github.com/mirceaulinic/latency-monitor/
  [complexipy]: https://github.com/rohaquinlop/complexipy
  [httptap]: https://github.com/ozeranskii/httptap
  [rust-ping]: https://github.com/karthik558/Rust-Ping
  [network-monitor]: https://github.com/grigio/network-monitor
  [waitress]: https://github.com/Pylons/waitress
  [flask]: https://github.com/pallets/flask
  [victoriametrics]: https://github.com/victoriametrics/VictoriaMetrics
  [perses]: https://github.com/perses/perses
  [openobserve]: https://github.com/openobserve/openobserve
  [smallstep]: https://github.com/smallstep/certificates
  [librenms]: https://github.com/librenms/librenms
  [wasmtime]: https://github.com/bytecodealliance/wasmtime
  [edgepython]: https://github.com/dylan-sutton-chavez/edge-python
  [absurd]: https://github.com/earendil-works/absurd
  [croniter]: https://github.com/pallets-eco/croniter
  [rtk]: https://github.com/rtk-ai/rtk
  [apscheduler]: https://github.com/agronholm/apscheduler
  [opentofu]: https://github.com/opentofu/opentofu
  [ansible]: https://github.com/ansible/ansible
  [yamlfmt]: https://github.com/google/yamlfmt
  [grpc]: https://github.com/grpc/grpc
  [rumdl]: https://github.com/rvben/rumdl
  [taws]: https://github.com/huseyinbabal/taws
  [deno]: https://github.com/denoland/deno
  [gobackup]: https://github.com/gobackup/gobackup
  [jujitsu]: https://github.com/jj-vcs/jj
  [podman]: https://github.com/podman-container-tools/podman
  [stockpiler]: https://github.com/lykinsbd/stockpiler
  [frr]: https://github.com/frrouting/frr
  [echarts]: https://github.com/apache/echarts
  [vuejs]: https://github.com/vuejs/core
  [stress-ng]: https://github.com/ColinIanKing/stress-ng
  [ruff]: https://github.com/astral-sh/ruff
  [htop]: https://github.com/htop-dev/htop
  [rustnet]: https://github.com/domcyrus/rustnet
  [procs]: https://github.com/dalance/procs
  [rustdesk]: https://github.com/rustdesk/rustdesk
  [rustloguru]: https://github.com/j-raghavan/rust-loguru
  [inject-python]: https://github.com/ivankorobkov/python-inject
  [goph]: https://github.com/melbahja/goph
  [hickory-dns]: https://github.com/hickory-dns/hickory-dns
  [lettre]: https://github.com/lettre/lettre
  [sniffglue]: https://github.com/kpcyrd/sniffglue
  [textfsm-rust]: https://github.com/joshbenz/textfsm-rust
  [rustin30minutes]: https://fasterthanli.me/articles/a-half-hour-to-learn-rust
  [rustix]: https://github.com/bytecodealliance/rustix
  [rustfortheimpatient]: https://youtu.be/br3GIIQeefY?is=bR4Q50xZ8Mx-kvOo
  [slint]: https://github.com/slint-ui/slint
  [evilhelix]: https://github.com/usagi-flow/evil-helix
  [atuin]: https://github.com/atuinsh/atuin
  [chrono]: https://github.com/chronotope/chrono
  [parking_lot]: https://github.com/Amanieu/parking_lot
  [colored]: https://github.com/colored-rs/colored
  [comprehensive_rust]: https://google.github.io/comprehensive-rust/
  [rustyline]: https://github.com/kkawakam/rustyline
  [rustpbx]: https://github.com/restsend/rustpbx
  [pestparser]: https://github.com/pest-parser/pest
