<template>
  <div class="flex flex-col items-center gap-6 w-full p-10">
    <!-- Welcome message -->
    <div class="hero border">
      <div class="hero-content flex-col">
        <div>
          <h1 class="text-5xl font-bold">Ethereum account info viewer</h1>
          <p class="py-6">Description text.</p>
        </div>
      </div>
    </div>

    <!-- Address input -->
    <div class="border w-full justify-center">
      <div class="form-control">
        <span class="label-text">
          1. What is your Ethereum account address?</span
        >
        <input
          v-model="address"
          type="text"
          placeholder="0x0000000000000000000000000000000000000000"
          class="input input-bordered"
        />
      </div>
    </div>

    <!-- Use cases navigation -->
    <div class="tabs border w-full justify-center">
      <!-- Tabs header -->
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          route.name == 'transactions' ? 'tab-active' : '',
        ]"
        to="/transactions"
        >Transactions</NuxtLink
      >
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          route.name == 'ether-balance' ? 'tab-active' : '',
        ]"
        to="/ether-balance"
        >Ether Balance</NuxtLink
      >
      <NuxtLink
        :class="[
          'tab',
          'tab-lifted',
          route.name == 'tokens-balance' ? 'tab-active' : '',
        ]"
        to="/tokens-balance"
        >Tokens Balance</NuxtLink
      >
    </div>

    <!-- Use case specific content -->
    <div class="border w-full">
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
