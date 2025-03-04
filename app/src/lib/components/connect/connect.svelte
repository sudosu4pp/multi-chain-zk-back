<script lang="ts">
import { setMode } from "mode-watcher"
import { derived } from "svelte/store"
import { navigating } from "$app/stores"
import Sun from "virtual:icons/lucide/sun"
import Moon from "virtual:icons/lucide/moon"
import Connection from "./connection.svelte"
import { cn } from "$lib/utilities/shadcn.ts"
import { Label } from "$lib/components/ui/label"
import * as Sheet from "$lib/components/ui/sheet"
import { Switch } from "$lib/components/ui/switch"
import { Button } from "$lib/components/ui/button"
import * as Avatar from "$lib/components/ui/avatar"
import WalletIcon from "virtual:icons/lucide/wallet"
import { crtEffectEnabled, showTokenDetails } from "$lib/stores/user.ts"
import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
import { sepoliaStore, evmWalletsInformation } from "$lib/wallet/evm/index.ts"
import { aptosStore, aptosWalletsInformation } from "$lib/wallet/aptos/index.ts"
import { cosmosStore, cosmosWalletsInformation } from "$lib/wallet/cosmos/index.ts"
import { onMount } from "svelte"
import DevSettings from "$lib/components/DevSettings/index.svelte"

let buttonText: string

let connectedWallets = derived(
  [sepoliaStore, cosmosStore, aptosStore],
  ([$sepoliaStore, $cosmosStore, $aptosStore]) => {
    return [
      $sepoliaStore.connectionStatus,
      $cosmosStore.connectionStatus,
      $aptosStore.connectionStatus
    ].filter(status => status === "connected").length
  }
)

// Set to 3 when shipping aptos.
const WALLET_COUNT = 2

$: if ($connectedWallets >= 1) {
  buttonText =
    $connectedWallets < WALLET_COUNT
      ? `Connected ${$connectedWallets}/${WALLET_COUNT}`
      : "Connected"
} else {
  buttonText = "Connect Wallet"
}

let sheetOpen = false
$: if ($navigating) sheetOpen = false
</script>

<Sheet.Root bind:open={sheetOpen}>
  <Sheet.Trigger asChild class="w-full" let:builder>
    <Button
      builders={[builder]}
      class={cn(
        $connectedWallets === 1 ? "w-[75px]" : "w-[50px]",
        "space-x-1.5 lg:w-[180px] text-md bg-accent text-black ml-auto",
        "hover:bg-cyan-300/90",
        $sepoliaStore.connectionStatus === "connected" &&
          $cosmosStore.connectionStatus === "connected",
      )}
      on:click={() => (sheetOpen = !sheetOpen)}
      size="sm"
    >
      <WalletIcon class="size-6 text-black" />
      <span class="font-supermolot font-bold uppercase lg:block hidden">
        {buttonText}
      </span>
      <span class={cn($connectedWallets === 1 ? "font-supermolot font-bold uppercase" : "hidden")}>
        <!-- {connectedWallets === 1 ? "1/2" : ""} -->
        {$connectedWallets === 3 ? "" : $connectedWallets > 1 ? `${$connectedWallets}/3` : ""}
      </span>
    </Button>
  </Sheet.Trigger>
  <Sheet.Content
    class={cn(
      "h-full border-solid border-left flex flex-col justify-start",
      "min-w-[95%] max-w-[90%] sm:min-w-min sm:max-w-[500px]",
      "overflow-y-auto",
    )}
  >
    <Sheet.Header>
      <Sheet.Title class="flex gap-4 items-center">
        <!-- Connect Wallet -->
        <Avatar.Root
          class={cn("size-10", $sepoliaStore.connectionStatus !== "connected" && "hidden")}
        >
          <Avatar.Image
            alt="ethereum avatar"
            src={`https://effigy.im/a/${$sepoliaStore.address || "0x8478B37E983F520dBCB5d7D3aAD8276B82631aBd"}.png`}
          />
          <Avatar.Fallback>UN</Avatar.Fallback>
        </Avatar.Root>
        <h2 class=" text-start w-full text-2xl font-bold uppercase font-supermolot">
          Connect Wallets
        </h2>
      </Sheet.Title>
    </Sheet.Header>
    <!-- Uncomment when shipping Aptos !-->
    <!--
    <Connection
      address={$aptosStore.address}
      chain="aptos"
      chainWalletsInformation={aptosWalletsInformation}
      connectStatus={$aptosStore.connectionStatus}
      connectedWalletId={$aptosStore.connectedWallet}
      hoverState={$aptosStore.hoverState}
      onConnectClick={aptosStore.connect}
      onDisconnectClick={aptosStore.disconnect}
    />
    !-->
    <Connection
      address={$sepoliaStore.address}
      chain="evm"
      chainWalletsInformation={evmWalletsInformation}
      connectStatus={$sepoliaStore.connectionStatus}
      connectedWalletId={$sepoliaStore.connectedWallet}
      hoverState={$sepoliaStore.hoverState}
      onConnectClick={sepoliaStore.connect}
      onDisconnectClick={sepoliaStore.disconnect}
    />
    <Connection
      address={$cosmosStore.address}
      chain="cosmos"
      chainWalletsInformation={cosmosWalletsInformation}
      connectStatus={$cosmosStore.connectionStatus}
      connectedWalletId={$cosmosStore.connectedWallet}
      hoverState={$cosmosStore.hoverState}
      onConnectClick={cosmosStore.connect}
      onDisconnectClick={cosmosStore.disconnect}
    />
    <DevSettings />
    <div class="flex justify-end w-full">
      <DropdownMenu.Root>
        <DropdownMenu.Trigger asChild let:builder>
          <Button
            builders={[builder]}
            variant="default"
            size="icon"
            class="hover:text-black hover:bg-accent"
          >
            <Sun
              class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
            />
            <Moon
              class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
            />
            <span class="sr-only">Toggle theme</span>
          </Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="w-fit rounded-none bg-secondary">
          <DropdownMenu.Group>
            <DropdownMenu.Item on:click={() => setMode("system")} class="cursor-pointer">
              System
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={() => setMode("dark")} class="cursor-pointer">
              Dark
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={() => setMode("light")} class="cursor-pointer">
              Light
            </DropdownMenu.Item>
            <DropdownMenu.Item>
              <div class="flex items-center space-x-2">
                <Switch bind:checked={$crtEffectEnabled} id="crt-effect-enabled" />
                <Label for="crt-effect-enabled">CRT effect</Label>
              </div>
            </DropdownMenu.Item>
          </DropdownMenu.Group>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </div>
  </Sheet.Content>
</Sheet.Root>
