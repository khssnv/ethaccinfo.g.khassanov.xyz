<template>
  <div>
    <div class="form-control m-2">
      <span class="m-2">From what block to start?</span>
      <input
        v-model="fromBlock"
        type="text"
        placeholder="0"
        class="input input-bordered m-2"
      />
      <button class="btn m-2" @click="query">Show</button>
    </div>
    <div class="m-4">
      <div v-if="!pending && queryCount > 0">
        <div v-if="transactions.length > 0">
          <div>Total {{ transactions.length }}</div>
          <li v-for="tx in transactions" :key="tx.transactionIndex">
            {{ tx }}
          </li>
        </div>
        <div v-else>
          This account made no transactions after block {{ fromBlock }}.
        </div>
      </div>
      <div v-else-if="pending">Loading...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();
const address = useState("address");
const fromBlock = useState("fromBlock", () => 0);

const useActionPath = (action) =>
  "/api" +
  "?module=account" +
  `&action=${action}` +
  `&address=${address.value}` +
  `&startblock=${fromBlock.value}` +
  "&endblock=99999999" +
  // "&page=1" +
  // "&offset=10" +
  "&sort=desc";

const transactions = ref([]);
const queryCount = ref(0);
const pending = ref(false);

async function query() {
  queryCount.value++;
  pending.value = true;

  const { data: normalTxs } = await useFetch(() => useActionPath("txlist"), {
    baseURL: config.public.apiBase,
    transform: (resp) => {
      return resp.result.map((tx) => {
        return { ...tx, kind: "normal" };
      });
    },
  });

  const { data: internalTxs } = await useFetch(
    () => useActionPath("txlistinternal"),
    {
      baseURL: config.public.apiBase,
      transform: (resp) => {
        return resp.result.map((tx) => {
          return { ...tx, kind: "internal" };
        });
      },
    }
  );

  const { data: erc20Txs } = await useFetch(() => useActionPath("tokentx"), {
    baseURL: config.public.apiBase,
    transform: (resp) => {
      return resp.result.map((tx) => {
        return { ...tx, kind: "erc20" };
      });
    },
  });

  const { data: erc721Txs } = await useFetch(
    () => useActionPath("tokennfttx"),
    {
      baseURL: config.public.apiBase,
      transform: (resp) => {
        return resp.result.map((tx) => {
          return { ...tx, kind: "erc721" };
        });
      },
    }
  );

  const { data: erc1155Txs } = await useFetch(
    () => useActionPath("token1155tx"),
    {
      baseURL: config.public.apiBase,
      transform: (resp) => {
        return resp.result.map((tx) => {
          return { ...tx, kind: "erc1155" };
        });
      },
    }
  );
  transactions.value = [
    ...(normalTxs.value || []),
    ...(internalTxs.value || []),
    ...(erc20Txs.value || []),
    ...(erc721Txs.value || []),
    ...(erc1155Txs.value || []),
  ]
    .map((tx) => {
      return { ...tx, ethUsed: tx.gas * tx.gasPrice * Math.pow(10, -18) };
    })
    .sort((a, b) => a.confirmations - b.confirmations);

  pending.value = false;
}
</script>
