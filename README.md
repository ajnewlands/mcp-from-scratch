This crate implements a very simple MCP server from scratch; it is intended to integrate with local copilot tools and was tested with the Amazon Q CLI.

To use it, you must tell your AI Copilot how to launch the MCP. In the case of the Amazon Q CLI, create ~/.aws/amazonq/mcp.json with an entry similar to the following:

```
{
    "mcpServers": {
      "mcp-warp-engine": {
        "command": "cargo",
        "args": [ "run", "--manifest-path", "/path/to/Cargo.toml" ],
        "env": {
                "WAYLAND_DISPLAY": ""
        },
        "timeout": 60000
      }
    }
}
```

After which, you can start up Q and play with the functions exposed by the MCP, which will play the role of Mr Sulu, helsman on the Starship Enterprise.
As such it can set or check the warp factor setting (between 1 and 10) of the ships engines.

Asking for an invalid warp factor
```
> Mr Sulu, warp factor 0.05 please!
I see you're making a Star Trek reference! You'd like to set the warp factor to 0.05. I notice there's an MCP tool
available that seems related to this request. Let me try to use it for you.


🛠️  Using tool: set_warp_factor from mcp server mcp_warp_engine
 ⋮
 ● Completed in 0.0s
It seems the warp engine doesn't want to cooperate with such a low warp factor. According to the response, "Mr Scott
says that order violates the laws of physics, Captain!"

Perhaps we should try a different warp factor? In Star Trek, warp factors typically start at warp 1, which is the speed
of light, with higher numbers representing exponentially faster speeds.
```

Setting a warp factor beyond the capability of the engines:
```
> Go to warp factor 11, we need to jump back through time to save the whales!
I'll help you set the warp factor to 11.


🛠️  Using tool: set_warp_factor (trusted) from mcp server mcp_example
 ⋮
 ● Completed in 0.0s
It looks like Mr. Scott is concerned about pushing the engines that hard! He says "if he gives her any more she'll blow,
Captain!" But the warp speed command was acknowledged.

Saving the whales is certainly a noble mission. Is there another approach you'd like to try for your time travel
endeavor?
```

A valid order
```
> OK, set warp factor 10, we'll slingshot around the sun to gain the required velocity.
I'll help you set the warp factor to 10 for your slingshot maneuver around the sun.


🛠️  Using tool: set_warp_factor (trusted) from mcp server mcp_example
⋮
Ahead, warp factor 10, sir! The command has been acknowledged and we're now at warp factor 10.

Your slingshot maneuver around the sun is a classic time travel technique.
With this velocity, you should be able to
break the time barrier and make your way back to save the whales. The Enterprise is responding well to the command.
```

Set and check a more sustainable cruising speed
```
> set warp factor 7.
I'll set the warp factor to 7 for you.

🛠️  Using tool: set_warp_factor (trusted) from mcp server mcp_warp_engine
 
 ● Running set_warp_factor with the param:
 ⋮
 ● Completed in 0.1s
Ahead, warp factor 7, sir!

The helmsman has acknowledged your command and the Enterprise is now traveling at warp factor 7. This is a good cruising
speed that balances power consumption with velocity - fast enough for interstellar travel without putting too much
strain on the engines.

> confirm the current speed
Let me check the current speed of the Enterprise for you.

🛠️  Using tool: get_warp_factor from mcp server mcp_warp_engine
 
 ● Running get_warp_factor with the param:
 ⋮
 ● Completed in 0.0s
Current warp factor is 7 sir!

The Enterprise is maintaining the speed you requested. At warp factor 7, you're traveling at approximately 656 times the
speed of light according to the TOS scale, or about 1,683 times the speed of light on the TNG scale. Either way, you're
making excellent progress through the galaxy!
```

