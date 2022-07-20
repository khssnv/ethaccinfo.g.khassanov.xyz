<template>
  <div class="flex flex-col items-center gap-6 w-full p-10">
    <!-- Welcome message -->
    <div class="hero">
      <div class="hero-content flex-col">
        <div>
          <h1 class="text-5xl font-bold">Ethereum account info viewer</h1>
          <p class="py-6">
            It can show all the data from account transactions, Ether and ERC-20
            tokens balances at specified date.
          </p>
        </div>
      </div>
    </div>

    <!-- Address input -->
    <div class="w-full justify-center p-4">
      <div class="form-control">
        <span class="m-2">What is your Ethereum account address?</span>
        <input
          v-model="address"
          type="text"
          placeholder="0x0000000000000000000000000000000000000000"
          class="input input-bordered m-2"
        />
      </div>
    </div>

    <!-- Use cases navigation -->
    <div class="tabs w-full justify-center p-4">
      <!-- Tabs header -->
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          'text-xl',
          route.name == 'transactions' ? 'tab-active' : '',
        ]"
        to="/transactions"
        >Transactions</NuxtLink
      >
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          'text-xl',
          route.name == 'ether-balance' ? 'tab-active' : '',
        ]"
        to="/ether-balance"
        >Ether Balance</NuxtLink
      >
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          'text-xl',
          route.name == 'tokens-balance' ? 'tab-active' : '',
        ]"
        to="/tokens-balance"
        >Tokens Balance</NuxtLink
      >
    </div>

    <!-- Use case specific content -->
    <div class="w-full p-4">
      <NuxtPage />
    </div>
  </div>
</template>

<script setup lang="ts">
const colorMode = useColorMode();
colorMode.preference = "lofi";

const route = useRoute();

const address = useState("address");
</script>
