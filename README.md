# DebakeEngine
Open-source DOP framework game engine build-in Rust

## 项目说明

## 安装说明

> ### 开发环境:
> #### 对于Windows平台开发者:
> - 项目主逻辑由Rust语言开发,C/C++语言作为项目动态链接库开发语言.
> - 需要你的电脑上安装Visual Studio(高于2017 community版本),因为我们需要你的电脑上拥有正确的MSVC环境并设置正确路径,该路径下须包含link.exe.[->如何在Windows中安装Cargo](#)
> - 如果你对Visual Studio恨之入骨,那么请移步[(对于Linux平台开发者)](#),一样的步骤也可以正常编译.
> - 由于项目所依赖的shaderc库需要在运行时编译,因此需要在你的电脑里面下载CMake(高于3.17版本),并确保CMake存在于你的环境变量中并为其设置正确的路径;[->如何设置Windows平台环境变量](#)
> - 同理,请确保Git存在于你的环境变量中并为其设置正确的路径;
> - 请确保项目根目录中包含vulkan-1.dll文件,项目主要使用Vulkan作为渲染层底层API.
> #### 对于Linux平台开发者:
> - 项目主逻辑由Rust语言开发,C/C++作为辅助动态链接库开发,因此需要你的电脑上拥有正确的GNU环境,并保证g++/gcc/gdb等可执行程序位于root:/bin文件夹中[->如何安装Cargo](#)
> - 由于 项目所依赖的shaderc库 需要在运行时编译,因此需要在你的电脑里面下载CMake(高于3.17版本),并确保CMake存在于root:/opt中;[->如何在Linux环境中安装指定版本CMake](#)
> - 同理,请确保已经正确安装了Git;
> #### 对于目标平台为Android的开发者:
> - 未来将有计划为移动平台开发
> #### 对于目标平台为IOS的开发者:
> - Well,I hate Metal API.Of cause I will be very greatful if you can add Metal API surrport for project.

## dse0.10说明:
> - ### Feature List
> - 功能集可以启用游戏引擎的大部分功能以及初始设置，如日志输出模式、目标编译平台、初始分辨率、静态宏、序列化模式、标准库使用等等.为了帮助减少编译时间，请考虑根据项目需求选择合适子选项或禁用选项.
> 
>| feature name   | description | sub feature |
>| -------------- | ----------- | ----------- |
>| debake         |             |             |
>| example        |             |             |
>| config         |             |             |
>| test           |             |             |
>| graphic_api    |             |             |
>| dse_macro      |             |             |
>| env            |             |             |
>| log_mode       |             |             |
>| main_test      |             |             |
>| vk_debug       |             |             |
>| resolution     |             |             |
>| root_path      |             |             |
>| serialize_mode |             |             |
>| std_use        |             |             |
>| execute        |             |             |

> - #### Config List
> - 配置集可以设定大部分初始设置，如日志输出模式、目标编译平台、初始分辨率、静态宏、序列化模式、标准库使用等等.为了帮助减少编译时间，请考虑根据项目需求选择合适子选项或禁用选项.
>
>| config name                   | description |   option    |
>| ------------------------------| ----------- | ----------- |
>|config_META_VALUE_TYPE_NUM|||
>|config_META_ARRAY_MAX_IMPL_NUM|||
>|config_DATUM_DEFAULT_CAPACITY|||
>|config_MAX_LOGGER_BUFFER_LEN|||
>|config_EXECUTE_SUB_STEP_LEN|||
>|config_TASK_DEFAULT_QUEUE_LEN|||
>|config_ENGINE_VERTEX_BUFFER|||
>|config_DEFAULT_INPUT_DETECT_OFFSET_MS|||
>|config_INPUT_UNIT_BUFFER_LEN|||
>|config_INPUT_BUFFER_DIM_NUM|||
>|config_UNIFORM_INPUT_BUFFER_CLEAR_COUNT_PER_FRAME|||
>|config_SOURCE_SHADER_TYPE|||