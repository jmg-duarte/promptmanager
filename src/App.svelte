<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Textarea, Input, Label } from "flowbite-svelte";
  import { LockSolid } from "flowbite-svelte-icons";

  let apiKey = "";
  let imageSource =
    "https://cdn.openai.com/labs/images/%22A%20sea%20otter%20with%20a%20pearl%20earring%22%20by%20Johannes%20Vermeer.webp?v=1";

  let prompt: string = "Test prompt";
  let prompts: string[] = new Array(50).fill("Text");

  async function submitPrompt() {
    if (prompt == "") {
      return;
    }

    invoke("generate_image", { apiToken: apiKey, prompt: prompt })
      .then((b64Image) => {
        console.log(b64Image);
        imageSource = `data:image/png;base64, ${b64Image}`;
      })
      .catch((error) => console.error(error));

    prompts = [prompt, ...prompts];
    prompt = "Test";
  }
</script>

<main class="flex flex-col overflow-hidden h-full">
  <!-- Header -->
  <div class="h-16 flex items-center border-b px-4">
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
  </div>
  <!-- Header -->

  <!-- PromptManager -->
  <div class="flex-1 flex overflow-hidden min-h-0">
    <ul
      class="flex-1 overflow-y-auto divide-y"
      style="min-width:30%; max-height:100%"
    >
      {#each prompts as prompt}
        <li
          class="px-2 py-2"
          style="word-wrap: break-word; white-space: pre-line;"
        >
          {prompt}
        </li>
      {/each}
    </ul>
    <div class="p-4" style="max-width:50%; max-height:100%;">
      {#if imageSource != ""}
        <img
          class="object-contain h-full w-full"
          src={imageSource}
          alt="Prompt Result"
        />
      {/if}
    </div>
  </div>
  <!-- PromptManager -->

  <!-- Prompt Input -->
  <div class="h-24 flex items-center px-4 space-x-4 border-t">
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
  <!-- Prompt Input -->
</main>
