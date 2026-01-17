# Useful Things

## Programming Infrastructure

- [vulture][vulture] - Find dead python code
- [pylama][pylama] - Useful python code-audit tool
- [pyupgrade][pyupgrade] - Automatically upgrade python syntax to baseline against specific python versions
- [yamllint][yamllint] - Simple yaml linter
- [yamlfix][yamlfix] - Automatic yaml formatting fixer
- [pudb][pudb] - Full-screen terminal Python debugger
- [pdoc][pdoc] - Documentation system for moderate-sized python projects
- [ALE][ale] - Add IDE features to `vim` and `neovim`
- [Neovide][neovide] - Enhanced neovim GUI with a focus on vim as an IDE 
- [nvim-treesitter][nvim-treesitter] - Parser and auto-complettion for neovim
- [YouCompleteMe][youcompleteme] - Auto-completion for ALE
- [PySnooper][pysnooper] - Debug python with decorators
- [snoop][snoop] - Debug python with decorators; similar to PySnooper
- [bandit][bandit] - Find security problems in python code
- [pss][pss] - Code-search tool written in python
- [rust-analyzer][rust-analyzer] - Linter for rust projects

### Profilers

- [pyinstrument][pyinstrument] - Call stack profiler for python libraries; detect why python is slow
- [py-spy][py-spy] - Sampling profiler for python code

### GitHub Actions

- [act][act]: Run GitHub actions locally on your laptop
- [zizmor][zizmor]: Static YAML analysis for GitHub Actions YAML

### Cloud Infrastructure

- [checkov][checkov]: Lint your AWS CloudFormation
- [ScoutSuite][ScoutSuite]: Multi-cloud security auditing

## Python Language

### Python - General Usage

- [pre-commit][pre-commit] - Add hooks to git commit actions
- [ultisnips][ultisnips]
- [cookiecutter][cookiecutter] - A disk and file templating framework
- [pyautogui][pyautogui] - Framework to exert control over GUI interfaces
- [pywinauto][pywinauto] - Framework to exert control over GUI interfaces
- [diagrams][diagrams] - Diagrams as Python code
- [mermaid-py][mermaid-py] - Python interface to `mermaid-js` diagramming library
- [jupyter-notebook][jupyter_notebook] - Dynamically analyze python code as a savable notebook

### Python - Compilers / Transpilers

- [codon][codon] - Compile Python into static binaries
- [PyO3][pyo3] - Write Python modules in rust, or embed Python in a rust binary
- [Cython][Cython] - Accelerate Python with C
- [py2many][py2many] - A python transpiler (i.e. convert Python to Rust, C++, etc...)
- [pyinstaller][pyinstaller] - Compile to binary including required packages
- [Nuitka][nuitka] - Compile python code to a binary
- [Cython][Cython] - CPython compiler

### Python - Code Libraries

- [Arrow][arrow] - Flexible Python time-handling
- [whenever][whenever] - Time-handling replacement for Python datetime and some [Arrow][arrow] use-cases
- [traitlets][traitlets] - Python classes with strong typing and attribute observer operations.
- [attrs][attrs] - Python classes without boilerplate
- [GitPython][gitpython] - Manage git repositories with python
- [loguru][loguru] - Delgan's excellent Python logging framework
- [rich][rich] - Library for terminal colorization
- [scrapy][scrapy] - Web scraping framework
- [marshmallow][marshmallow] - Implement Python object serialization an de-serialization with type validation; works well with nested dicts
- [trio][trio] - Python async and concurrency library
- [orjson][orjson] - Fast and correct json parsing library
- [pydhcpd][pydhcpd] - Fast Python DHCPd
- [ptftpd][ptftpd] - Python TFTPd and PXE tool suite
- [pyjwt][pyjwt] - Python implementation of JWT (JSON Web Token)
- [python-saml][python-saml] - Python SAML toolkit
- [dnspython][dnspython] - Python interface to DNS queries
- [splunk-sdk-python][splunk-sdk-python] - Splunk SDK for Python
- [hvac][hvac] - Python interface to Hashicorp Vault
- [pyshark][pyshark] - Python interface to `tshark` / Wireshark in the Terminal
- [docker-py][docker-py] - Python API for the Docker ecosystem
- [peewee][peewee] - Python ORM for sqllite, postgresql, mysql and cockroach DB
- [records][records] - Make SQL queries and ingest results as python objects
- [python-prompt-toolkit][python-prompt-toolkit] - For interactive python terminal prompt apps
- [psutil][psutil] - Cross platform process and system monitoring
- [watchdog][watchdog] - Monitor file-system events in python
- [invoke][invoke] - Pythonic command execution and task management
- [pandas][pandas] - Pythonic data manipulation and statistical analysis library
- [polars][polars] - Python data manipulation and statistical analysis written in rust
- [Seaborn][seaborn] - Simple statistical graphing package
- [plotly][plotly] - Graphing and plotting library
- [prophet][prophet] - Facebook's canned time-series forcasting library
- [pysmb][pysmb] - Python SMB client
- [pyyaml][pyyaml] - Pythonic yaml handling
- [ruamel.yaml][ruamel-yaml] - Pythonic yaml handling
- [rocketry][rocketry] - Pythonic scheduling and cron-like recur using python decorators
- [pexpect][pexpect] - Pythonic Expect library
- [pyparsing][pyparsing] - Popular python parsing library via PEG parser
- [TextFSM][textfsm] - Stateful templated python text parsing and value extraction
- [transitions][transitions] - Python Finite State Machine
- [fuzzywuzzy][fuzzywuzzy] - Fuzzy text matching
- [click][click] - Flexible CLI arguments library as an improvement over `argparse`
- [rich][rich] - Command-line text formatting library
- [python-prompt-toolkit][python_prompt_toolkit] - Build interactive Python command-line programs
- [deepdiff][deepdiff] - Diff python nested dicts
- [dictdiffer][dictdiffer] - Diff python dicts
- [dotenv][dotenv] - Manage `.env` files in Python
- [transitions][transitions] - Object-oriented pythonic State Machine
- [dominate][dominate] - Write HTML with python
- [matplotlib][matplotlib] - Build graphs with Python
- [seaborn][seaborn] - Build statistical graphs with Python
- [PyGithub][PyGithub] - GitHub v3 API python library
- [GitPython][GitPython] - Python API for git repositories
- [atlassian-python-api][atlassian-python-api] - REST API for Atlassian products (i.e. Jira, Confluence)


### Python - Network Automation

- [fabric][fabric] - Automate command execution on linux systems via SSH
- [netmiko][netmiko] - Automate command execution on routers / switches via SSH
- [exabgp][exabgp] - BGPd written in Python
- [nornir][nornir] - Network inventory and execution abstractions
- [ntc-templates][ntc-templates] - Templates to parse command output via [TextFSM][textfsm]
- [scrapli][scrapli] - Automate command execution on routers / switches via SSH
- [suzieq][suzieq] - Canned network metrics collection
- [meraki-cli][meraki-cli] - Meraki CLI tool for interacting with the dashboard
- [py-junos-eznc][py-junos-eznc] - Junos automation library 
- [boto3][boto3] - Automate AWS environments

## Machine Learning

- [keras][keras]
- [scikit-learn][scikit-learn]
- [langchain][langchain] - Framework for building AI Agents
- [Text-Blob][Text-Blog] - Text processing and sentiment analysis
- [MLAlgorithms][MLAlgorithms] - Clean Machine Learning examples

## Rust Language

The best single-source of rust examples is [Rust By Example][rust-by-example].

### CLI Argument parsing

- [argh][argh] - Simplified Rust CLI arg parsing
- [clap][clap] - Full-featured Rust CLI arg parsing; can be complex and often deprecates previously-used syntax

### Error Handling

- [snafu][snafu] - Rust error handling
- [anyhow][anyhow] - Rust Application error handling
- [thiserror][thiserror] - Rust Application error handling

### General libraries

- [rexpect][rexpect] - Rust expect library


## Tools

### General Use utilities

- [difftastic][difftastic]: A text file differ that understands many programming languages.  Diffs are intelligent based on the language in the file.
- [taskwarrior][taskwarrior]: A task-list manager with a useful CLI
- [yamlfix][yamlfix]: YAML formatter and fixer
- [just][just]: `just` is like `make`, but without the `Makefile` syntax warts
- [sd][sd]: Find and replace simplified
- [ripgrep][ripgrep]: Recursive grep through directories

### Terminal utilites

- [asciienema][asciienema]: A terminal session recorder built with Rust
- [obs-studio][obs-studio]: Record YouTube videos from your laptop

### Backend utilites

- [influxdb][influxdb]: A time-series database
- [rrdtool][rrdtool]: A time-series database, which intelligently compresses data

### Network Tools - Services

- [caddy][caddy]: Go webserver / reverse proxy with auto-TLS certificate generation
- [Netbox][netbox]: Network source of truth; based on Django
- [NIPAP][nipap]: Open-source IP address management (IPAM)
- [fail2ban][fail2ban]: Monitor and black-hole repetitive attacks on your Linux server
- [maltrail][maltrail]: Detect malicious network traffic
- [RustScan][RustScan]: Port-scanner; like `nmap` but in Rust


### Network Tools - Diagnostics and Testing

- [trippy][trippy]: Traceroute / [mtr][mtr] written in Rust
- [mtr][mtr]: Traceroute / mtr written in Rust
- [sniffnet][sniffnet]: Cross-platform network traffic capture with a nice GUI (Windows / MacOS / Linux)
- [gping][gping]: Graph ping response times (written in Rust) in the terminal window
- [pingtracer][pingtracer]: Progressive ping logger written for MS Windows; discovers all hops in the path and pings them
- [nethogs][nethogs]: Detect which linux processes are consuming the most bandwidth
- [bandwhich][bandwhich]: Detect which linux processes are consuming the most bandwidth (written in Rust)
- [sniffer][sniffer]: Detect which linux / Windows processes are consuming the most bandwidth (written in Go)
- [mitmproxy][mitmproxy]: An interactive TLS-capable intercepting HTTP proxy for penetration testers and software developers.
- [go-mitmproxy][go-mitmproxy]: A port of [mitmproxy][mitmproxy] to Go
- [flent][flent]: A network performance test client, which requires [fping][fping] (client-side), [netperf][netperf] (server-side) or [iperf2][iperf2] (server-side)
- [netperf][netperf]: A network performance test server, written by Hewlett Packard.  It's best to install the Debian package instead of trying to compile [netperf][netperf]


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
  [seaborn]: https://github.com/mwaskom/seaborn
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
  [nuitka]: https://github.com/Nuitka/Nuitka
  [peewee]: https://github.com/coleifer/peewee
  [psutil]: https://github.com/giampaolo/psutil
  [MLAlgorithms]: https://github.com/rushter/MLAlgorithms
  [Cython]: https://github.com/cython/cython
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
  [transitions]: https://github.com/pytransitions/transitions
  [pywinauto]: https://github.com/pywinauto/pywinauto
  [attrs]: https://github.com/python-attrs/attrs
  [pyjwt]: https://github.com/jpadilla/pyjwt
  [GitPython]: https://github.com/gitpython-developers/GitPython
  [yamllint]: https://github.com/adrienverge/yamllint
  [yamlfix]: https://github.com/lyz-code/yamlfix
  [pudb]: https://github.com/inducer/pudb
  [pexpect]: https://github.com/pexpect/pexpect
  [dnspython]: https://github.com/rthalley/dnspython
  [pyshark]: https://github.com/KimiNewt/pyshark
  [pydoc]: https://github.com/mitmproxy/pdoc
  [pyparsing]: https://github.com/pyparsing/pyparsing
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
  [rrdtool]: https://github.com/oetiker/rrdtool-1.x
  [just]: https://github.com/casey/just
  [netbox]: https://github.com/netbox-community/netbox
  [nipap]: https://github.com/SpriteLink/NIPAP 
  [yamlfix]: https://github.com/lyz-code/yamlfix
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

