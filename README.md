# Lapce Copilot
This is an unofficial Copilot plugin for Lapce.   

## Installation
Requires: NodeJS 16+
It will try to find the `node` binary in your path, but you can also change the setting to point to the binary.
  
- Open Lapce
- Go to the plugins panel
- Find 'Copilot (Unofficial)'
- Click install

This will immediately open a browser window to the Github login page with a notification containing a code. Enter that code into the website to authorize Copilot. You shouldn't need to do this again.

## Updating
If Copilot ends up out of date, then it can be updated by copying the `dist/` folder from the [copilot.vim](https://github.com/github/copilot.vim/) repo. That repo contains an agent.js which this plugin starts as the actual core copilot.  

## Impl Details
This plugin currently:
- Looks for node
    - And checks if the `node` version is good
- Tells Lapce to spawn a new Language Server using roughly `[node, agent.js]`
    - I'm not sure if it is waiting for copilot to send the initialize response back to Lapce, possibly it should
- The plugin sends to the newly spawned Copilot LSP information about the editor + plugin
    - names and versions, but also some configuration
- The plugin asks Copilot if we are signed in
    - If we are not, then it does a few more requests to try signing in
- The Copilot LSP currently uses a nonstandard `getCompletions`/`getCompletionsCycling` request. 
    - I didn't try to implement this in Lapce because it is from a single plugin, and is also of dubious origin since Github doesn't document their own API.
    - Lapce implements the 3.18 (upcoming) LSP command `textDocument/inlineCompletion` which serves a similar purpose.
    - So the plugin tells Lapce that it supports Inline Completions, and maps those to the Copilot LSP requests and back.
    - We also have to listen for onChange/onOpen events, because Copilot wants the `version` of the file to be sent with the request but `textDocument/inlineCompletion` does not include that.

Once 3.18 is standardized, if Copilot's agent.js implements inlineCompletion then that special-handling can be removed from this extension.

## License  
The license of the *plugin* is Apache/MIT, but the license of the files in the `dist/` is covered under the [GitHub Terms of Service](https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features#github-copilot). Possibly the definitions in `copilot.rs` of the RPC commands also falls under Github's license?