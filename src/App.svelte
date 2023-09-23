<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Textarea, Input, Label } from "flowbite-svelte";
  import { LockSolid } from "flowbite-svelte-icons";
  let apiKey = "";

  import { OpenAI } from "openai";

  let imageSource =
    "https://cdn.openai.com/labs/images/%22A%20sea%20otter%20with%20a%20pearl%20earring%22%20by%20Johannes%20Vermeer.webp?v=1";

  let prompt: string = "";
  let prompts: string[] = [];

  async function submitPrompt() {
    if (prompt == "") {
      return;
    }

    // const openai = new OpenAI({
    //   apiKey: apiKey,
    //   dangerouslyAllowBrowser: true,
    // });
    // const image = await openai.images.generate({
    //   prompt: prompt,
    // });

    // Decent sidestep but probably not amazing
    invoke("generate_image", { apiToken: apiKey, prompt: prompt })
      .then((b64Image) => {
        console.log(b64Image);
        imageSource = `data:image/png;base64, ${b64Image}`;
      })
      .catch((error) => console.error(error));
    prompts = [prompt, ...prompts];
    prompt = "";
  }
</script>

<main class="flex-column h-screen w-screen" style="min-width:800px;">
  <header
    class="h-[5vh] flex p-4 items-center border-b"
    style="min-height:48px"
  >
    <h1 class="text-xl flex-1">PromptManager</h1>

    <div class="flex items-center space-x-4">
      <div>
        <Label>OpenAI API Key</Label>
      </div>
      <div>
        <Input bind:value={apiKey} type="text">
          <LockSolid
            slot="left"
            class="w-4 h-4 text-gray-500 dark:text-gray-400"
          />
        </Input>
      </div>
    </div>
  </header>
  <div class="h-[95vh] flex flex-col pb-4 pr-4 pl-4">
    <div class="flex flex-1 overflow-hidden space-x-4">
      <div class="flex-1 overflow-auto" style="min-width:30%; max-width:50%;">
        {#each prompts as prompt, idx}
          <p class="p-2" style="word-wrap: break-word; white-space: pre-line;">
            {prompt}
          </p>
          {#if idx != prompts.length - 1}
            <hr />
          {/if}
        {/each}
      </div>
      <div class="flex place-content-center" style="max-width:50%">
        {#if imageSource != ""}
          <img
            class="aspect-square object-contain rounded-lg"
            src={imageSource}
            alt="Prompt Result"
          />
        {/if}
      </div>
    </div>
    <div class="flex items-center space-x-4" style="min-height:64px">
      <Textarea
        bind:value={prompt}
        class="flex-1"
        style="resize:none;"
        placeholder="Lakeside view of the mountains..."
      />
      <Button
        color="blue"
        disabled={prompt == "" || apiKey == ""}
        on:click={submitPrompt}>Submit</Button
      >
    </div>
  </div>
</main>
