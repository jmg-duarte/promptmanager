<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Body, Client, ResponseType, getClient } from "@tauri-apps/api/http";
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

    // invoke("generate_image", { apiToken: apiKey, prompt: prompt })
    //   .then((b64Image) => {
    //     console.log(b64Image);
    //     imageSource = `data:image/png;base64, ${b64Image}`;
    //   })
    //   .catch((error) => console.error(error));

    prompts = [prompt, ...prompts];
    prompt = "Test";
  }
</script>

<main>
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

  <!-- Prompt Input -->
  <div class="h-24 flex items-center px-4 space-x-4">
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

  <div class="">
    <div class="flex space-x-4 px-4">
      <div class="flex-1">
        <ul class="overflow-y-scroll divide-y" style="min-width:30%;">
          {#each prompts as prompt}
            <li
              class="py-2"
              style="word-wrap: break-word; white-space: pre-line;"
            >
              {prompt}
            </li>
          {/each}
        </ul>
      </div>
      <div class="flex" style="max-width:50%">
        {#if imageSource != ""}
          <div>
            <img
              class="aspect-square object-contain rounded-lg"
              src={imageSource}
              alt="Prompt Result"
            />
          </div>
        {/if}
      </div>
    </div>
  </div>
</main>
