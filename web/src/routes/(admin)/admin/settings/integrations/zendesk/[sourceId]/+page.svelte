<script lang="ts">
    import { enhance } from '$app/forms'
    import { Button } from '$lib/components/ui/button'
    import { Input } from '$lib/components/ui/input'
    import { Label } from '$lib/components/ui/label'
    import { Switch } from '$lib/components/ui/switch'
    import { Checkbox } from '$lib/components/ui/checkbox'
    import * as Card from '$lib/components/ui/card'
    import * as Alert from '$lib/components/ui/alert'
    import { AlertCircle, Loader2, LifeBuoy } from '@lucide/svelte'
    import { onMount } from 'svelte'
    import { beforeNavigate } from '$app/navigation'
    import type { PageProps } from './$types'
    import type { ZendeskSourceConfig } from '$lib/types'

    let { data }: PageProps = $props()

    const cfg = data.source.config as ZendeskSourceConfig

    let enabled = $state(data.source.isActive)
    let syncTickets = $state(cfg.sync_tickets ?? true)
    let syncHelpCenter = $state(cfg.sync_help_center ?? true)

    let email = $state(data.principalEmail ?? '')
    let apiToken = $state('')

    let isSubmitting = $state(false)
    let formErrors = $state<string[]>([])
    let skipUnsavedCheck = $state(false)

    let beforeUnloadHandler: ((e: BeforeUnloadEvent) => void) | null = null

    const origEnabled = data.source.isActive
    const origSyncTickets = cfg.sync_tickets ?? true
    const origSyncHelpCenter = cfg.sync_help_center ?? true
    const origEmail = data.principalEmail ?? ''

    const hasUnsavedChanges = $derived(
        enabled !== origEnabled ||
            syncTickets !== origSyncTickets ||
            syncHelpCenter !== origSyncHelpCenter ||
            email !== origEmail ||
            apiToken !== '',
    )

    onMount(() => {
        beforeUnloadHandler = (e: BeforeUnloadEvent) => {
            if (hasUnsavedChanges && !skipUnsavedCheck) {
                e.preventDefault()
                e.returnValue = ''
            }
        }
        window.addEventListener('beforeunload', beforeUnloadHandler)
        return () => {
            if (beforeUnloadHandler) {
                window.removeEventListener('beforeunload', beforeUnloadHandler)
            }
        }
    })

    beforeNavigate(({ cancel }) => {
        if (hasUnsavedChanges && !skipUnsavedCheck) {
            const shouldLeave = confirm(
                'You have unsaved changes. Are you sure you want to leave this page?',
            )
            if (!shouldLeave) cancel()
        }
    })

    function validateForm(): boolean {
        formErrors = []
        if (!syncTickets && !syncHelpCenter) {
            formErrors = [...formErrors, 'Select at least one thing to sync']
        }
        if (apiToken && !email.trim()) {
            formErrors = [...formErrors, 'Admin email is required when updating the API token']
        }
        if (!apiToken && email.trim() !== origEmail) {
            formErrors = [...formErrors, 'Enter the API token to change the admin email']
        }
        return formErrors.length === 0
    }
</script>

<svelte:head>
    <title>Configure Zendesk - {data.source.name}</title>
</svelte:head>
{#if formErrors.length > 0}
    <Alert.Root variant="destructive">
        <AlertCircle class="h-4 w-4" />
        <Alert.Title>Configuration Error</Alert.Title>
        <Alert.Description>
            <ul class="list-inside list-disc">
                {#each formErrors as err (err)}
                    <li>{err}</li>
                {/each}
            </ul>
        </Alert.Description>
    </Alert.Root>
{/if}

<form
    method="POST"
    use:enhance={() => {
        if (!validateForm()) return async () => {}
        isSubmitting = true
        return async ({ result, update }) => {
            if (result.type === 'redirect') {
                skipUnsavedCheck = true
                if (beforeUnloadHandler) {
                    window.removeEventListener('beforeunload', beforeUnloadHandler)
                    beforeUnloadHandler = null
                }
            }
            await update()
            isSubmitting = false
        }
    }}>
    <Card.Root>
        <Card.Header>
            <div class="flex items-start justify-between">
                <div>
                    <Card.Title class="flex items-center gap-2">
                        <LifeBuoy class="h-5 w-5" />
                        {data.source.name}
                    </Card.Title>
                    <Card.Description class="mt-1">
                        Index Zendesk Support tickets and Help Center articles (read-only)
                    </Card.Description>
                </div>
                <div class="flex items-center gap-2">
                    <Label for="enabled" class="text-sm">Enabled</Label>
                    <Switch
                        id="enabled"
                        bind:checked={enabled}
                        name="enabled"
                        class="cursor-pointer" />
                </div>
            </div>
        </Card.Header>

        <Card.Content class="space-y-6">
            <div class="space-y-4">
                <h3 class="text-sm font-semibold">Connection</h3>

                <div class="space-y-1.5">
                    <Label for="subdomain">Zendesk subdomain</Label>
                    <Input id="subdomain" value={cfg.subdomain} readonly disabled />
                    <p class="text-muted-foreground text-xs">
                        https://{cfg.subdomain}.zendesk.com. To use a different Zendesk account,
                        create a new source.
                    </p>
                </div>

                <div class="space-y-2">
                    <div class="flex items-center gap-2">
                        <Checkbox id="syncTickets" name="syncTickets" bind:checked={syncTickets} />
                        <Label for="syncTickets" class="cursor-pointer">Sync tickets</Label>
                    </div>
                    <div class="flex items-center gap-2">
                        <Checkbox
                            id="syncHelpCenter"
                            name="syncHelpCenter"
                            bind:checked={syncHelpCenter} />
                        <Label for="syncHelpCenter" class="cursor-pointer"
                            >Sync Help Center articles</Label>
                    </div>
                </div>
            </div>

            <div class="space-y-4 border-t pt-4">
                <h3 class="text-sm font-semibold">Credentials</h3>

                <div class="space-y-1.5">
                    <Label for="email">Admin email</Label>
                    <Input
                        id="email"
                        name="email"
                        type="email"
                        bind:value={email}
                        placeholder="admin@example.com"
                        autocomplete="username" />
                </div>

                <div class="space-y-1.5">
                    <Label for="apiToken">
                        API token
                        <span class="text-muted-foreground ml-1 font-normal">
                            (leave blank to keep current)
                        </span>
                    </Label>
                    <Input
                        id="apiToken"
                        name="apiToken"
                        type="password"
                        bind:value={apiToken}
                        placeholder="Enter a new token to update"
                        autocomplete="new-password" />
                    <p class="text-muted-foreground text-xs">
                        Created in Admin Center under Apps and integrations → APIs → Zendesk API, by
                        an admin user.
                    </p>
                </div>
            </div>
        </Card.Content>

        <Card.Footer class="flex justify-end">
            <Button
                type="submit"
                disabled={isSubmitting || !hasUnsavedChanges}
                class="cursor-pointer">
                {#if isSubmitting}
                    <Loader2 class="mr-2 h-4 w-4 animate-spin" />
                {/if}
                Save Configuration
            </Button>
        </Card.Footer>
    </Card.Root>
</form>
